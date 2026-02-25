use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractSolid {
    pub(crate) abstract_geometric_primitive: AbstractGeometricPrimitive,
}

impl AbstractSolid {
    pub fn new(abstract_geometric_primitive: AbstractGeometricPrimitive) -> Self {
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
                use $crate::model::geometry::primitives::AsAbstractSolid;
                &self.abstract_solid().abstract_geometric_primitive
            }
        }

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut for $type {
            fn abstract_geometric_primitive_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractGeometricPrimitive {
                use $crate::model::geometry::primitives::AsAbstractSolidMut;
                &mut self.abstract_solid_mut().abstract_geometric_primitive
            }
        }
    };
}

impl_abstract_solid_traits!(AbstractSolid);
