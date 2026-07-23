use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractSolid {
    pub abstract_geometric_primitive: AbstractGeometricPrimitive,
}

impl AbstractSolid {
    pub fn new(abstract_geometric_primitive: AbstractGeometricPrimitive) -> Self {
        Self {
            abstract_geometric_primitive,
        }
    }

    pub fn from_abstract_geometric_primitive(
        abstract_geometric_primitive: AbstractGeometricPrimitive,
    ) -> Self {
        Self {
            abstract_geometric_primitive,
        }
    }
}

pub trait AsAbstractSolid: AsAbstractGeometricPrimitive {
    fn abstract_solid(&self) -> &AbstractSolid;
}

pub trait AsAbstractSolidMut: AsAbstractSolid + AsAbstractGeometricPrimitiveMut {
    fn abstract_solid_mut(&mut self) -> &mut AbstractSolid;
}

impl AsAbstractSolid for AbstractSolid {
    fn abstract_solid(&self) -> &AbstractSolid {
        self
    }
}

impl AsAbstractSolidMut for AbstractSolid {
    fn abstract_solid_mut(&mut self) -> &mut AbstractSolid {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_solid_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometric_primitive_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitive for $type {
            fn abstract_geometric_primitive(
                &self,
            ) -> &$crate::model::geometry::primitives::AbstractGeometricPrimitive {
                &<$type as $crate::model::geometry::primitives::AsAbstractSolid>::abstract_solid(
                    self,
                )
                .abstract_geometric_primitive
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_solid_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometric_primitive_mut_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut for $type {
            fn abstract_geometric_primitive_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractGeometricPrimitive {
                &mut <$type as $crate::model::geometry::primitives::AsAbstractSolidMut>::abstract_solid_mut(self)
                    .abstract_geometric_primitive
            }
        }
    };
}

impl_abstract_solid_traits!(AbstractSolid);
impl_abstract_solid_mut_traits!(AbstractSolid);
