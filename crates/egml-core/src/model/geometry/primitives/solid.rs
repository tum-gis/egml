use crate::error::Error;
use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::primitives::{
    AbstractSolid, AsAbstractSolid, AsAbstractSolidMut, Shell, TriangulatedSurface,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{impl_abstract_solid_mut_traits, impl_abstract_solid_traits, impl_has_geometry_type};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::prelude::*;

/// A 3-D geometry bounded by one or more surfaces.
///
/// Corresponds to `gml:Solid` in [OGC 07-036 §10.6.4](https://docs.ogc.org/is/07-036/07-036.pdf).  The bounding surfaces are
/// stored as [`Shell`] members and may be of any [`AbstractSurfaceKind`](crate::model::geometry::primitives::AbstractSurfaceKind).
#[derive(Debug, Clone, PartialEq)]
pub struct Solid {
    pub abstract_solid: AbstractSolid,
    exterior: Option<Shell>,
    interior: Vec<Shell>,
}

impl Solid {
    /// Creates a new `Solid` from its bounding surfaces.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `members` is empty.
    pub fn new(exterior: Option<Shell>) -> Result<Self, Error> {
        Ok(Self {
            abstract_solid: AbstractSolid::default(),
            exterior,
            interior: Vec::new(),
        })
    }

    pub fn from_abstract_solid(abstract_solid: AbstractSolid, exterior: Option<Shell>) -> Self {
        Self {
            abstract_solid,
            exterior,
            interior: Vec::new(),
        }
    }

    pub fn exterior(&self) -> Option<&Shell> {
        self.exterior.as_ref()
    }

    pub fn interior(&self) -> &[Shell] {
        &self.interior
    }

    pub fn set_interior(&mut self, interior: Vec<Shell>) {
        self.interior = interior;
    }

    pub fn push_interior(&mut self, interior: Shell) {
        self.interior.push(interior);
    }

    pub fn extend_interiors(&mut self, interiors: impl IntoIterator<Item = Shell>) {
        self.interior.extend(interiors);
    }
}

impl Solid {
    pub fn points(&self) -> Vec<&DirectPosition> {
        self.exterior
            .as_ref()
            .map(|exterior| exterior.points())
            .unwrap_or_default()
    }

    /// Returns the volume of this solid.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingExteriorShell`] if the solid has no exterior shell property.
    /// Propagates any error from triangulating the bounding surfaces.
    pub fn volume_3d(&self) -> Result<f64, Error> {
        self.exterior
            .as_ref()
            .ok_or(Error::MissingExteriorShell)?
            .volume_3d()
    }
}

impl ApplyTransform for Solid {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(exterior) = self.exterior.as_mut() {
            exterior.apply_transform(transform);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_transform(transform));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(exterior) = self.exterior.as_mut() {
            exterior.apply_isometry(isometry);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(exterior) = self.exterior.as_mut() {
            exterior.apply_translation(vector);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(exterior) = self.exterior.as_mut() {
            exterior.apply_rotation(rotation);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(exterior) = self.exterior.as_mut() {
            exterior.apply_scale(scale);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_scale(scale));
    }
}

impl ComputeEnvelope for Solid {
    /// Returns the union of the bounding boxes of all surface members.
    fn compute_envelope(&self) -> Option<Envelope> {
        self.exterior.as_ref()?.compute_envelope()
    }
}

impl AsAbstractSolid for Solid {
    fn abstract_solid(&self) -> &AbstractSolid {
        &self.abstract_solid
    }
}

impl AsAbstractSolidMut for Solid {
    fn abstract_solid_mut(&mut self) -> &mut AbstractSolid {
        &mut self.abstract_solid
    }
}

impl_abstract_solid_traits!(Solid);
impl_abstract_solid_mut_traits!(Solid);
impl_has_geometry_type!(Solid, Solid);

impl IterGeometries for Solid {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(
                    self.exterior
                        .as_ref()
                        .into_iter()
                        .flat_map(|x| x.iter_geometries()),
                )
                .chain(self.interior.iter().flat_map(|x| x.iter_geometries())),
        )
    }
}

impl Triangulate for Solid {
    /// Triangulates the exterior shell and all interior shells into a single
    /// [`TriangulatedSurface`] covering the full boundary of this solid.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingExteriorShell`] if the solid has no exterior shell.
    /// Propagates any error from [`Shell::triangulate`].
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let exterior_shell = self.exterior.as_ref().ok_or(Error::MissingExteriorShell)?;

        let (exterior_surface, mut skipped) = exterior_shell.triangulate()?.into_parts();
        let mut surfaces = vec![exterior_surface];

        for shell in &self.interior {
            let (surface, nested_skipped) = shell.triangulate()?.into_parts();
            surfaces.push(surface);
            skipped.extend(nested_skipped);
        }

        let combined = TriangulatedSurface::from_triangulated_surfaces(surfaces)?;
        Ok(Triangulation::new(combined, skipped))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::Envelope;

    #[test]
    fn volume_3d_unit_cube() {
        let solid = Envelope::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
        )
        .unwrap()
        .to_solid()
        .unwrap();
        assert!((solid.volume_3d().unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn volume_3d_2x3x4_box() {
        let solid = Envelope::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(2.0, 3.0, 4.0).unwrap(),
        )
        .unwrap()
        .to_solid()
        .unwrap();
        assert!((solid.volume_3d().unwrap() - 24.0).abs() < 1e-10);
    }

    #[test]
    fn apply_translation_transforms_interior_shells_too() {
        let mut solid = Envelope::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
        )
        .unwrap()
        .to_solid()
        .unwrap();

        let interior_shell = solid.exterior().cloned().unwrap();
        solid.push_interior(interior_shell);

        let before = solid.interior()[0].points()[0].x();

        solid.apply_translation(Vector3::new(5.0, 0.0, 0.0));

        let after = solid.interior()[0].points()[0].x();

        assert!((after - before - 5.0).abs() < 1e-10);
    }

    #[test]
    fn iter_geometries_walks_shell_polygons_and_rings() {
        let solid = Envelope::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
        )
        .unwrap()
        .to_solid()
        .unwrap();

        // self (1) + exterior shell (1) + 6 faces (6) + 6 rings, one per face (6) = 14
        assert_eq!(solid.iter_geometries().count(), 14);
    }

    #[test]
    fn volume_3d_missing_exterior_shell() {
        let solid = Solid::new(None).unwrap();
        assert_eq!(solid.volume_3d(), Err(Error::MissingExteriorShell));
    }
}
