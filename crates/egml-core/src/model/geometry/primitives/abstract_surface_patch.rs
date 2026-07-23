use crate::model::AbstractObject;

/// Base data shared by all GML surface-patch types ([OGC 07-036 §10.5.12.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// A surface patch is a sub-surface used as a building block inside a
/// [`Surface`](crate::model::geometry::primitives::Surface).  Concrete
/// patch types are [`PolygonPatch`] and [`Triangle`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AbstractSurfacePatch {
    pub abstract_object: AbstractObject,
}

impl AbstractSurfacePatch {
    pub fn new() -> Self {
        Self {
            abstract_object: AbstractObject::default(),
        }
    }

    pub fn from_abstract_object(abstract_object: AbstractObject) -> Self {
        Self { abstract_object }
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

#[macro_export]
macro_rules! impl_abstract_surface_patch_traits {
    ($type:ty) => {
        impl $crate::model::AsAbstractObject for $type {
            fn abstract_object(&self) -> &$crate::model::AbstractObject {
                &<$type as $crate::model::geometry::primitives::AsAbstractSurfacePatch>::abstract_surface_patch(self).abstract_object
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_surface_patch_mut_traits {
    ($type:ty) => {
        impl $crate::model::AsAbstractObjectMut for $type {
            fn abstract_object_mut(&mut self) -> &mut $crate::model::AbstractObject {
                &mut <$type as $crate::model::geometry::primitives::AsAbstractSurfacePatchMut>::abstract_surface_patch_mut(
                    self,
                )
                .abstract_object
            }
        }
    };
}

impl_abstract_surface_patch_traits!(AbstractSurfacePatch);
impl_abstract_surface_patch_mut_traits!(AbstractSurfacePatch);
