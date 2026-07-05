use crate::Error;
use crate::model::geometry::primitives::{SurfaceProperty, TriangulatedSurface};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::{Isometry3, Vector3};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct Shell {
    members: Vec<SurfaceProperty>,
}

impl Shell {
    pub fn new(members: impl IntoIterator<Item = SurfaceProperty>) -> Result<Self, Error> {
        let members: Vec<SurfaceProperty> = members.into_iter().collect();
        if members.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:Solid",
                minimum: 1,
                spec: Some("OGC 07-036 §10.6.4"),
                id: None,
                detail: None,
            });
        }

        Ok(Self { members })
    }

    /// Returns the bounding surface members of this solid.
    pub fn members(&self) -> &[SurfaceProperty] {
        &self.members
    }

    /// Replaces the bounding surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `val` is empty.
    pub fn set_members(&mut self, val: Vec<SurfaceProperty>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:Solid",
                minimum: 1,
                spec: Some("OGC 07-036 §10.6.4"),
                id: None,
                detail: None,
            });
        }
        self.members = val;
        Ok(())
    }

    pub fn push_member(&mut self, member: SurfaceProperty) {
        self.members.push(member);
    }

    pub fn extend_members(&mut self, members: impl IntoIterator<Item = SurfaceProperty>) {
        self.members.extend(members);
    }
}

impl Shell {
    /// Triangulates all bounding surfaces and merges them into a single [`TriangulatedSurface`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TriangulationFailed`] if any bounding surface cannot be triangulated.
    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surfaces: Vec<TriangulatedSurface> = self
            .members
            .iter()
            .flat_map(|x| x.object.as_ref())
            .map(|x| x.triangulate())
            .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

        let combined_triangulated_surface =
            TriangulatedSurface::from_triangulated_surfaces(triangulated_surfaces)?;
        Ok(combined_triangulated_surface)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        self.members
            .iter()
            .flat_map(|x| x.object.as_ref())
            .fold(Vec::new(), |mut acc, x| {
                acc.extend(x.points().iter());
                acc
            })
    }

    /// Returns the volume of the closed solid bounded by this shell.
    ///
    /// Uses the divergence theorem on the triangulated shell: `V = |Σ a·(b×c)| / 6`
    /// per triangle. The shell must be closed and watertight for the result to be correct.
    ///
    /// # Errors
    ///
    /// Propagates any [`Error::TriangulationFailed`] from triangulating the bounding surfaces.
    pub fn volume_3d(&self) -> Result<f64, Error> {
        let triangulated_surface = self.triangulate()?;
        let triangles = triangulated_surface.triangles();
        let Some(first) = triangles.first() else {
            return Ok(0.0);
        };

        // Shift all vertices by the first triangle's vertex to avoid catastrophic
        // cancellation when world-space coordinates are large (e.g. EPSG:25832).
        let origin: Vector3<f64> = first.a.into();
        let signed_vol: f64 = triangles
            .iter()
            .map(|t| {
                let a: Vector3<f64> = Into::<Vector3<f64>>::into(t.a) - origin;
                let b: Vector3<f64> = Into::<Vector3<f64>>::into(t.b) - origin;
                let c: Vector3<f64> = Into::<Vector3<f64>>::into(t.c) - origin;
                a.dot(&b.cross(&c))
            })
            .sum();
        Ok(signed_vol.abs() / 6.0)
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object.as_mut() {
                x.apply_transform(m);
            }
        });
    }

    /// Returns the union of the bounding boxes of all surface members.
    pub fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .members
            .iter()
            .flat_map(|x| x.object.as_ref())
            .flat_map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
    }
}
