use crate::Error;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{
    AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut, PolygonPatch,
    Triangle, TriangulatedSurface,
};
use nalgebra::Isometry3;

/// Discriminated union of all concrete surface-patch implementations.
#[derive(Debug, Clone, PartialEq)]
pub enum SurfacePatchKind {
    /// A general polygonal patch — [`PolygonPatch`].
    PolygonPatch(PolygonPatch),
    /// A triangular patch — [`Triangle`].
    Triangle(Triangle),
}

impl AsAbstractSurfacePatch for SurfacePatchKind {
    fn abstract_surface_patch(&self) -> &AbstractSurfacePatch {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.abstract_surface_patch(),
            SurfacePatchKind::Triangle(x) => x.abstract_surface_patch(),
        }
    }
}

impl AsAbstractSurfacePatchMut for SurfacePatchKind {
    fn abstract_surface_patch_mut(&mut self) -> &mut AbstractSurfacePatch {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.abstract_surface_patch_mut(),
            SurfacePatchKind::Triangle(x) => x.abstract_surface_patch_mut(),
        }
    }
}

impl SurfacePatchKind {
    pub fn area_3d(&self) -> Result<f64, Error> {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.area_3d(),
            SurfacePatchKind::Triangle(x) => Ok(x.area()),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.compute_envelope(),
            SurfacePatchKind::Triangle(x) => Some(x.compute_envelope()),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.apply_transform(m),
            SurfacePatchKind::Triangle(x) => x.apply_transform(m),
        }
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.triangulate(),
            SurfacePatchKind::Triangle(x) => x.triangulate(),
        }
    }
}
