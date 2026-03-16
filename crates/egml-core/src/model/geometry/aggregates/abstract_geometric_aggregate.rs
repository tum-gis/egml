use crate::model::geometry::{AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut};

/// Base data shared by all GML multi-geometry aggregate types (ISO 19136 §10.6).
///
/// An aggregate groups homogeneous geometry primitives without imposing
/// topological constraints between them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometricAggregate {
    pub(crate) abstract_geometry: AbstractGeometry,
}

impl AbstractGeometricAggregate {
    pub fn new(abstract_geometry: AbstractGeometry) -> Self {
        Self { abstract_geometry }
    }
}

/// Object-safe read accessor for [`AbstractGeometricAggregate`] fields.
pub trait AsAbstractGeometricAggregate: AsAbstractGeometry {
    /// Returns a reference to the embedded [`AbstractGeometricAggregate`] base data.
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate;
}

/// Mutable companion to [`AsAbstractGeometricAggregate`].
pub trait AsAbstractGeometricAggregateMut:
    AsAbstractGeometricAggregate + AsAbstractGeometryMut
{
    /// Returns a mutable reference to the embedded [`AbstractGeometricAggregate`] base data.
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

#[doc(hidden)]
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
