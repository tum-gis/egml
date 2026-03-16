use crate::Error;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{PolygonPatch, Triangle, TriangulatedSurface};
use nalgebra::Isometry3;

/// Base data shared by all GML surface-patch types (ISO 19136 §10.5.5).
///
/// A surface patch is a sub-surface used as a building block inside a
/// [`Surface`](crate::model::geometry::primitives::Surface).  Concrete
/// patch types are [`PolygonPatch`] and [`Triangle`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AbstractSurfacePatch {}

impl AbstractSurfacePatch {
    pub fn new() -> Self {
        Self {}
    }
}

/// Object-safe read accessor for [`AbstractSurfacePatch`] fields.
pub trait AsAbstractSurfacePatch {
    /// Returns a reference to the embedded [`AbstractSurfacePatch`] base data.
    fn abstract_surface_patch(&self) -> &AbstractSurfacePatch;
}

/// Mutable companion to [`AsAbstractSurfacePatch`].
pub trait AsAbstractSurfacePatchMut: AsAbstractSurfacePatch {
    /// Returns a mutable reference to the embedded [`AbstractSurfacePatch`] base data.
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
