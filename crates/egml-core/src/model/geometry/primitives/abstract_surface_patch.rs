use crate::Error;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{PolygonPatch, Triangle, TriangulatedSurface};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AbstractSurfacePatch {}

impl AbstractSurfacePatch {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait AsAbstractSurfacePatch {
    fn abstract_surface_patch(&self) -> &AbstractSurfacePatch;
}

pub trait AsAbstractSurfacePatchMut: AsAbstractSurfacePatch {
    fn abstract_surface_patch_mut(&mut self) -> &mut AbstractSurfacePatch;
}

impl AsAbstractSurfacePatch for AbstractSurfacePatch {
    fn abstract_surface_patch(&self) -> &AbstractSurfacePatch {
        self
    }
}

impl AsAbstractSurfacePatchMut for AbstractSurfacePatch {
    fn abstract_surface_patch_mut(&mut self) -> &mut AbstractSurfacePatch {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SurfacePatchKind {
    PolygonPatch(PolygonPatch),
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
    pub fn area(&self) -> f64 {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.area(),
            SurfacePatchKind::Triangle(x) => x.area(),
        }
    }

    pub fn compute_envelope(&self) -> Envelope {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.compute_envelope(),
            SurfacePatchKind::Triangle(x) => x.compute_envelope(),
        }
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        match self {
            SurfacePatchKind::PolygonPatch(x) => x.triangulate(),
            SurfacePatchKind::Triangle(x) => x.triangulate(),
        }
    }
}
