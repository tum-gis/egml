use crate::model::geometry::{AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut};

/// Base data shared by all GML multi-geometry aggregate types ([OGC 07-036 §11.3.1.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// An aggregate groups homogeneous geometry primitives without imposing
/// topological constraints between them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometricAggregate {
    pub abstract_geometry: AbstractGeometry,
}

impl AbstractGeometricAggregate {
    pub fn new() -> Self {
        Self {
            abstract_geometry: AbstractGeometry::default(),
        }
    }

    pub fn from_abstract_geometry(abstract_geometry: AbstractGeometry) -> Self {
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

#[macro_export]
macro_rules! impl_abstract_geometric_aggregate_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometry_traits!($type);

        impl $crate::model::geometry::AsAbstractGeometry for $type {
            fn abstract_geometry(&self) -> &$crate::model::geometry::AbstractGeometry {
                &<$type as $crate::model::geometry::aggregates::AsAbstractGeometricAggregate>::abstract_geometric_aggregate(self)
                    .abstract_geometry
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_geometric_aggregate_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometry_mut_traits!($type);

        impl $crate::model::geometry::AsAbstractGeometryMut for $type {
            fn abstract_geometry_mut(&mut self) -> &mut $crate::model::geometry::AbstractGeometry {
                &mut <$type as $crate::model::geometry::aggregates::AsAbstractGeometricAggregateMut>::abstract_geometric_aggregate_mut(self)
                    .abstract_geometry
            }
        }
    };
}

impl_abstract_geometric_aggregate_traits!(AbstractGeometricAggregate);
impl_abstract_geometric_aggregate_mut_traits!(AbstractGeometricAggregate);
