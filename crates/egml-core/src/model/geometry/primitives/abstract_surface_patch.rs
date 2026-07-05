/// Base data shared by all GML surface-patch types ([OGC 07-036 §10.5.12.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
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
