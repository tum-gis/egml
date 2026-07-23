use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
};
/// Base data shared by all GML curve geometry types ([OGC 07-036 §10.4.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// A curve is a 1-D geometric primitive representing a connected series of
/// coordinate positions.  The only concrete curve type currently implemented
/// is [`LineString`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractCurve {
    pub abstract_geometric_primitive: AbstractGeometricPrimitive,
}

impl AbstractCurve {
    pub fn new() -> Self {
        Self {
            abstract_geometric_primitive: AbstractGeometricPrimitive::default(),
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

/// Object-safe read accessor for [`AbstractCurve`] fields.
pub trait AsAbstractCurve: AsAbstractGeometricPrimitive {
    /// Returns a reference to the embedded [`AbstractCurve`] base data.
    fn abstract_curve(&self) -> &AbstractCurve;
}

/// Mutable companion to [`AsAbstractCurve`].
pub trait AsAbstractCurveMut: AsAbstractCurve + AsAbstractGeometricPrimitiveMut {
    /// Returns a mutable reference to the embedded [`AbstractCurve`] base data.
    fn abstract_curve_mut(&mut self) -> &mut AbstractCurve;
}

impl AsAbstractCurve for AbstractCurve {
    fn abstract_curve(&self) -> &AbstractCurve {
        self
    }
}

impl AsAbstractCurveMut for AbstractCurve {
    fn abstract_curve_mut(&mut self) -> &mut AbstractCurve {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_curve_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometric_primitive_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitive for $type {
            fn abstract_geometric_primitive(
                &self,
            ) -> &$crate::model::geometry::primitives::AbstractGeometricPrimitive {
                &<$type as $crate::model::geometry::primitives::AsAbstractCurve>::abstract_curve(
                    self,
                )
                .abstract_geometric_primitive
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_curve_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometric_primitive_mut_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut for $type {
            fn abstract_geometric_primitive_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractGeometricPrimitive {
                &mut <$type as $crate::model::geometry::primitives::AsAbstractCurveMut>::abstract_curve_mut(self)
                    .abstract_geometric_primitive
            }
        }
    };
}

impl_abstract_curve_traits!(AbstractCurve);
impl_abstract_curve_mut_traits!(AbstractCurve);
