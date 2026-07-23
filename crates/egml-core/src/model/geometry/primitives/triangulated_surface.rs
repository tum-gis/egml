use crate::error::Error;
use crate::model::common::{ApplyTransform, ComputeEnvelope, IterGeometries};
use crate::model::geometry::primitives::abstract_surface_patch_kind::AbstractSurfacePatchKind;
use crate::model::geometry::primitives::{
    AbstractSurfacePatchArrayProperty, AsSurface, AsSurfaceMut, Surface, Triangle,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{impl_has_geometry_type, impl_surface_mut_traits, impl_surface_traits};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

/// A 2-D surface composed exclusively of [`Triangle`] patches.
///
/// Corresponds to `gml:TriangulatedSurface` in [OGC 07-036 §10.5.11.4](https://docs.ogc.org/is/07-036/07-036.pdf).
/// This type is the primary output of triangulation operations.
#[derive(Debug, Clone, PartialEq)]
pub struct TriangulatedSurface {
    surface: Surface,
}

impl TriangulatedSurface {
    /// Creates a new `TriangulatedSurface` from an existing [`Surface`].
    pub fn new(surface: Surface) -> Result<Self, Error> {
        Ok(TriangulatedSurface { surface })
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }
}

impl TriangulatedSurface {
    /// Creates a `TriangulatedSurface` from a flat list of triangles.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `triangles` is empty.
    pub fn from_triangles(triangles: Vec<Triangle>) -> Result<Self, Error> {
        Self::validate_triangles(&triangles)?;

        let patches: Vec<AbstractSurfacePatchKind> = triangles
            .into_iter()
            .map(AbstractSurfacePatchKind::Triangle)
            .collect();
        let surface_patch_array_property: AbstractSurfacePatchArrayProperty =
            AbstractSurfacePatchArrayProperty::from_objects(patches);

        Self::new(Surface::new(surface_patch_array_property))
    }

    /// Merges multiple triangulated surfaces into one by combining all their patches.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `surfaces` is empty.
    pub fn from_triangulated_surfaces(surfaces: Vec<TriangulatedSurface>) -> Result<Self, Error> {
        Self::validate_surfaces(&surfaces)?;

        let patches: Vec<AbstractSurfacePatchKind> = surfaces
            .into_iter()
            .flat_map(|surface| {
                let mut patch_array = surface.surface.into_patches();
                std::mem::take(patch_array.objects_mut())
            })
            .collect();

        let surface_patch_array_property: AbstractSurfacePatchArrayProperty =
            AbstractSurfacePatchArrayProperty::from_objects(patches);

        let surface = Surface::new(surface_patch_array_property);
        Ok(TriangulatedSurface { surface })
    }

    fn validate_triangles(triangles: &[Triangle]) -> Result<(), Error> {
        if triangles.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:TriangulatedSurface",
                minimum: 1,
                spec: Some("OGC 07-036 §10.5.11.4"),
                id: None,
                detail: None,
            });
        }
        Ok(())
    }

    fn validate_surfaces(surfaces: &[TriangulatedSurface]) -> Result<(), Error> {
        if surfaces.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "TriangulatedSurface::from_triangulated_surfaces",
                minimum: 1,
                spec: None,
                id: None,
                detail: None,
            });
        }
        Ok(())
    }

    /// Returns references to all [`Triangle`] patches in this surface.
    pub fn triangles(&self) -> Vec<&Triangle> {
        self.surface
            .patches()
            .objects()
            .iter()
            .filter_map(|patch| match patch {
                AbstractSurfacePatchKind::Triangle(triangle) => Some(triangle),
                _ => None,
            })
            .collect()
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        self.surface.points()
    }

    pub fn area_3d(&self) -> Result<f64, Error> {
        self.surface.area_3d()
    }
}

impl AsSurface for TriangulatedSurface {
    fn surface(&self) -> &Surface {
        &self.surface
    }
}

impl AsSurfaceMut for TriangulatedSurface {
    fn surface_mut(&mut self) -> &mut Surface {
        &mut self.surface
    }
}

impl_surface_traits!(TriangulatedSurface);
impl_surface_mut_traits!(TriangulatedSurface);
impl_has_geometry_type!(TriangulatedSurface, TriangulatedSurface);

impl IterGeometries for TriangulatedSurface {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()))
    }
}

impl ApplyTransform for TriangulatedSurface {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.surface.apply_transform(transform);
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.surface.apply_isometry(isometry);
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.surface.apply_translation(vector);
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.surface.apply_rotation(rotation);
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.surface.apply_scale(scale);
    }
}

impl ComputeEnvelope for TriangulatedSurface {
    /// Returns the axis-aligned bounding box of all triangles.
    fn compute_envelope(&self) -> Option<Envelope> {
        self.surface.compute_envelope()
    }
}
