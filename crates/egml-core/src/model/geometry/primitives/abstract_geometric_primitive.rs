use crate::model::geometry::{AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometricPrimitive {
    pub(crate) abstract_geometry: AbstractGeometry,
}

impl AbstractGeometricPrimitive {
    pub fn new(abstract_geometry: AbstractGeometry) -> Self {
        Self { abstract_geometry }
    }
}

pub trait AsAbstractGeometricPrimitive: AsAbstractGeometry {
    fn abstract_geometric_primitive(&self) -> &AbstractGeometricPrimitive;
}

pub trait AsAbstractGeometricPrimitiveMut:
    AsAbstractGeometricPrimitive + AsAbstractGeometryMut
{
    fn abstract_geometric_primitive_mut(&mut self) -> &mut AbstractGeometricPrimitive;
}

impl AsAbstractGeometricPrimitive for AbstractGeometricPrimitive {
    fn abstract_geometric_primitive(&self) -> &AbstractGeometricPrimitive {
        self
    }
}

impl AsAbstractGeometricPrimitiveMut for AbstractGeometricPrimitive {
    fn abstract_geometric_primitive_mut(&mut self) -> &mut AbstractGeometricPrimitive {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_geometric_primitive_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometry_traits!($type);

        impl $crate::model::geometry::AsAbstractGeometry for $type {
            fn abstract_geometry(&self) -> &$crate::model::geometry::AbstractGeometry {
                use $crate::model::geometry::primitives::AsAbstractGeometricPrimitive;
                &self.abstract_geometric_primitive().abstract_geometry
            }
        }

        impl $crate::model::geometry::AsAbstractGeometryMut for $type {
            fn abstract_geometry_mut(&mut self) -> &mut $crate::model::geometry::AbstractGeometry {
                use $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut;
                &mut self.abstract_geometric_primitive_mut().abstract_geometry
            }
        }
    };
}

impl_abstract_geometric_primitive_traits!(AbstractGeometricPrimitive);
