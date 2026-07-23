use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::primitives::{
    AbstractSurface, AbstractSurfaceProperty, AsAbstractSurface, AsAbstractSurfaceMut,
    TriangulatedSurface,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{
    Error, impl_abstract_surface_mut_traits, impl_abstract_surface_traits, impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct Shell {
    abstract_surface: AbstractSurface,
    members: Vec<AbstractSurfaceProperty>,
}

impl Shell {
    pub fn new(members: impl IntoIterator<Item = AbstractSurfaceProperty>) -> Result<Self, Error> {
        let members: Vec<AbstractSurfaceProperty> = members.into_iter().collect();
        Self::validate(&members)?;

        Ok(Self {
            abstract_surface: AbstractSurface::default(),
            members,
        })
    }

    pub fn from_abstract_surface(
        abstract_surface: AbstractSurface,
        members: impl IntoIterator<Item = AbstractSurfaceProperty>,
    ) -> Result<Self, Error> {
        let members: Vec<AbstractSurfaceProperty> = members.into_iter().collect();
        Self::validate(&members)?;

        Ok(Self {
            abstract_surface,
            members,
        })
    }

    fn validate(members: &[AbstractSurfaceProperty]) -> Result<(), Error> {
        if members.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:Solid",
                minimum: 1,
                spec: Some("OGC 07-036 §10.6.4"),
                id: None,
                detail: None,
            });
        }
        Ok(())
    }

    /// Returns the bounding surface members of this solid.
    pub fn members(&self) -> &[AbstractSurfaceProperty] {
        &self.members
    }

    /// Replaces the bounding surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `val` is empty.
    pub fn set_members(&mut self, val: Vec<AbstractSurfaceProperty>) -> Result<(), Error> {
        Self::validate(&val)?;

        self.members = val;
        Ok(())
    }

    pub fn push_member(&mut self, member: AbstractSurfaceProperty) {
        self.members.push(member);
    }

    pub fn extend_members(&mut self, members: impl IntoIterator<Item = AbstractSurfaceProperty>) {
        self.members.extend(members);
    }
}

impl AsAbstractSurface for Shell {
    fn abstract_surface(&self) -> &AbstractSurface {
        &self.abstract_surface
    }
}

impl AsAbstractSurfaceMut for Shell {
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface {
        &mut self.abstract_surface
    }
}

impl_abstract_surface_traits!(Shell);
impl_abstract_surface_mut_traits!(Shell);
impl_has_geometry_type!(Shell, Shell);

impl IterGeometries for Shell {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(
                self.members
                    .iter()
                    .filter_map(|x| x.object())
                    .flat_map(|x| x.iter_geometries()),
            ),
        )
    }
}

impl Shell {
    pub fn points(&self) -> Vec<&DirectPosition> {
        self.members
            .iter()
            .flat_map(|x| x.object())
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
        let triangulation = self.triangulate()?;
        let triangles = triangulation.surface().triangles();
        let Some(first) = triangles.first() else {
            return Ok(0.0);
        };

        // Shift all vertices by the first triangle's vertex to avoid catastrophic
        // cancellation when world-space coordinates are large (e.g. EPSG:25832).
        let origin: Vector3<f64> = first.a().into();
        let signed_vol: f64 = triangles
            .iter()
            .map(|t| {
                let a: Vector3<f64> = Into::<Vector3<f64>>::into(t.a()) - origin;
                let b: Vector3<f64> = Into::<Vector3<f64>>::into(t.b()) - origin;
                let c: Vector3<f64> = Into::<Vector3<f64>>::into(t.c()) - origin;
                a.dot(&b.cross(&c))
            })
            .sum();
        Ok(signed_vol.abs() / 6.0)
    }
}

impl ApplyTransform for Shell {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_transform(transform);
            }
        });
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_isometry(isometry);
            }
        });
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_translation(vector);
            }
        });
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_rotation(rotation);
            }
        });
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_scale(scale);
            }
        });
    }
}

impl ComputeEnvelope for Shell {
    /// Returns the union of the bounding boxes of all surface members.
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .members
            .iter()
            .flat_map(|x| x.object())
            .flat_map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

impl Triangulate for Shell {
    /// Members that fail to triangulate individually (e.g. a degenerate ring) are
    /// skipped rather than failing the whole shell; see their errors via
    /// [`Triangulation::skipped`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if no member could be triangulated.
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let mut surfaces = Vec::new();
        let mut skipped = Vec::new();

        for member in self.members.iter().flat_map(|x| x.object()) {
            match member.triangulate() {
                Ok(triangulation) => {
                    let (surface, nested_skipped) = triangulation.into_parts();
                    surfaces.push(surface);
                    skipped.extend(nested_skipped);
                }
                Err(error) => {
                    skipped.push(error);
                }
            }
        }

        let combined = TriangulatedSurface::from_triangulated_surfaces(surfaces)?;
        Ok(Triangulation::new(combined, skipped))
    }
}
