use crate::model::geometry::{AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometricAggregate {
    pub(crate) abstract_geometry: AbstractGeometry,
}

impl AbstractGeometricAggregate {
    pub fn new(abstract_geometry: AbstractGeometry) -> Self {
        Self { abstract_geometry }
    }
}

pub trait AsAbstractGeometricAggregate: AsAbstractGeometry {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate;
}

pub trait AsAbstractGeometricAggregateMut:
    AsAbstractGeometricAggregate + AsAbstractGeometryMut
{
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate;
}

impl AsAbstractGeometricAggregate for AbstractGeometricAggregate {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        self
    }
}

impl AsAbstractGeometricAggregateMut for AbstractGeometricAggregate {
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_geometric_aggregate_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometry_traits!($type);

        impl $crate::model::geometry::AsAbstractGeometry for $type {
            fn abstract_geometry(&self) -> &$crate::model::geometry::AbstractGeometry {
                &self.abstract_geometric_aggregate().abstract_geometry
            }
        }

        impl $crate::model::geometry::AsAbstractGeometryMut for $type {
            fn abstract_geometry_mut(&mut self) -> &mut $crate::model::geometry::AbstractGeometry {
                &mut self.abstract_geometric_aggregate_mut().abstract_geometry
            }
        }
    };
}

impl_abstract_geometric_aggregate_traits!(AbstractGeometricAggregate);
