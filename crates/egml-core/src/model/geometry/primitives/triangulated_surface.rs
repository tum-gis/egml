use crate::error::Error;
// use crate::impl_surface_traits;
use crate::impl_surface_traits;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{
    AbstractSurface, AsSurface, AsSurfaceMut, Surface, SurfacePatchArrayProperty, SurfacePatchKind,
    Triangle,
};
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TriangulatedSurface {
    pub(crate) surface: Surface,
}

impl TriangulatedSurface {
    pub fn new(surface: Surface) -> Result<Self, Error> {
        Ok(TriangulatedSurface { surface })
    }

    pub fn from_triangles(triangles: Vec<Triangle>) -> Result<Self, Error> {
        if triangles.is_empty() {
            return Err(Error::MustNotBeEmpty("triangulated surface"));
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

    pub fn from_triangulated_surfaces(surfaces: Vec<TriangulatedSurface>) -> Result<Self, Error> {
        if surfaces.is_empty() {
            return Err(Error::MustNotBeEmpty("surfaces to combine"));
        }

        // TODO: without cloning
        let patches: Vec<SurfacePatchKind> = surfaces
            .into_iter()
            .flat_map(|surface| surface.patches().patches().clone())
            .collect();

        let surface_patch_array_property: SurfacePatchArrayProperty =
            SurfacePatchArrayProperty::new(patches);

        let surface = Surface::new(AbstractSurface::default(), surface_patch_array_property);
        Ok(TriangulatedSurface { surface })
    }

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

    pub fn compute_envelope(&self) -> Envelope {
        self.surface.compute_envelope()
    }

    /*pub fn patches(&self) -> &Vec<Triangle> {
        self.surface.patches()
    }

    pub fn append_patches(&mut self, mut patches: Vec<Triangle>) {
        self.patches.append(&mut patches)
    }*/

    /*pub fn points(&self) -> Vec<&DirectPosition> {
        self.patches.iter().fold(Vec::new(), |mut acc, x| {
            acc.extend(x.points().iter());
            acc
        })
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.patches.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }*/
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
