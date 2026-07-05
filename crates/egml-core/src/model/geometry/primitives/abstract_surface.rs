use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
};
/// Base data shared by all GML surface geometry types ([OGC 07-036 §10.5.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// A surface is a 2-D geometric primitive.  Concrete surface types include
/// [`Polygon`], [`Surface`], and [`CompositeSurface`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractSurface {
    pub(crate) abstract_geometric_primitive: AbstractGeometricPrimitive,
}

impl AbstractSurface {
    pub fn new(abstract_geometric_primitive: AbstractGeometricPrimitive) -> Self {
        Self {
            abstract_geometric_primitive,
        }
    }
}

/// Object-safe read accessor for [`AbstractSurface`] fields.
pub trait AsAbstractSurface: AsAbstractGeometricPrimitive {
    /// Returns a reference to the embedded [`AbstractSurface`] base data.
    fn abstract_surface(&self) -> &AbstractSurface;
}

/// Mutable companion to [`AsAbstractSurface`].
pub trait AsAbstractSurfaceMut: AsAbstractSurface + AsAbstractGeometricPrimitiveMut {
    /// Returns a mutable reference to the embedded [`AbstractSurface`] base data.
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface;
}

impl AsAbstractSurface for AbstractSurface {
    fn abstract_surface(&self) -> &AbstractSurface {
        self
    }
}

impl AsAbstractSurfaceMut for AbstractSurface {
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface {
        self
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_abstract_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometric_primitive_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitive for $type {
            fn abstract_geometric_primitive(
                &self,
            ) -> &$crate::model::geometry::primitives::AbstractGeometricPrimitive {
                use $crate::model::geometry::primitives::AsAbstractSurface;
                &self.abstract_surface().abstract_geometric_primitive
            }
        }

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut for $type {
            fn abstract_geometric_primitive_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractGeometricPrimitive {
                use $crate::model::geometry::primitives::AsAbstractSurfaceMut;
                &mut self.abstract_surface_mut().abstract_geometric_primitive
            }
        }
    };
}

impl_abstract_surface_traits!(AbstractSurface);
