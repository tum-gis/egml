use crate::error::Error;
use crate::impl_surface_traits;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{
    AbstractSurface, AsSurface, AsSurfaceMut, Surface, SurfacePatchArrayProperty, SurfacePatchKind,
    Triangle,
};
use nalgebra::Isometry3;
/// A 2-D surface composed exclusively of [`Triangle`] patches.
///
/// Corresponds to `gml:TriangulatedSurface` in ISO 19136 §10.5.14.
/// This type is the primary output of triangulation operations.
#[derive(Debug, Clone, PartialEq)]
pub struct TriangulatedSurface {
    pub(crate) surface: Surface,
}

impl TriangulatedSurface {
    /// Creates a new `TriangulatedSurface` from an existing [`Surface`].
    pub fn new(surface: Surface) -> Result<Self, Error> {
        Ok(TriangulatedSurface { surface })
    }

    /// Creates a `TriangulatedSurface` from a flat list of triangles.
    ///
    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if `triangles` is empty.
    pub fn from_triangles(triangles: Vec<Triangle>) -> Result<Self, Error> {
        if triangles.is_empty() {
            return Err(Error::EmptyCollection("triangulated surface"));
        }

        let patches: Vec<SurfacePatchKind> = triangles
            .into_iter()
            .map(SurfacePatchKind::Triangle)
            .collect();
        let surface_patch_array_property: SurfacePatchArrayProperty =
            SurfacePatchArrayProperty::new(patches);

        Self::new(Surface::new(
            AbstractSurface::default(),
            surface_patch_array_property,
        ))
    }

    /// Merges multiple triangulated surfaces into one by combining all their patches.
    ///
    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if `surfaces` is empty.
    pub fn from_triangulated_surfaces(surfaces: Vec<TriangulatedSurface>) -> Result<Self, Error> {
        if surfaces.is_empty() {
            return Err(Error::EmptyCollection("surfaces to combine"));
        }

        let patches: Vec<SurfacePatchKind> = surfaces
            .into_iter()
            .flat_map(|surface| surface.surface.into_patches().patches)
            .collect();

        let surface_patch_array_property: SurfacePatchArrayProperty =
            SurfacePatchArrayProperty::new(patches);

        let surface = Surface::new(AbstractSurface::default(), surface_patch_array_property);
        Ok(TriangulatedSurface { surface })
    }

    /// Returns references to all [`Triangle`] patches in this surface.
    pub fn triangles(&self) -> Vec<&Triangle> {
        self.surface
            .patches()
            .patches()
            .iter()
            .filter_map(|patch| match patch {
                SurfacePatchKind::Triangle(triangle) => Some(triangle),
                _ => None,
            })
            .collect()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.surface.apply_transform(m);
    }

    /// Returns the axis-aligned bounding box of all triangles.
    pub fn compute_envelope(&self) -> Envelope {
        self.surface.compute_envelope()
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
