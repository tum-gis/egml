use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
    LineString,
};
use nalgebra::Isometry3;

/// Base data shared by all GML curve geometry types (ISO 19136 §10.4.3).
///
/// A curve is a 1-D geometric primitive representing a connected series of
/// coordinate positions.  The only concrete curve type currently implemented
/// is [`LineString`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractCurve {
    pub(crate) abstract_geometric_primitive: AbstractGeometricPrimitive,
}

impl AbstractCurve {
    pub fn new(abstract_geometric_primitive: AbstractGeometricPrimitive) -> Self {
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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_abstract_curve_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometric_primitive_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitive for $type {
            fn abstract_geometric_primitive(
                &self,
            ) -> &$crate::model::geometry::primitives::AbstractGeometricPrimitive {
                use $crate::model::geometry::primitives::AsAbstractCurve;
                &self.abstract_curve().abstract_geometric_primitive
            }
        }

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut for $type {
            fn abstract_geometric_primitive_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractGeometricPrimitive {
                use $crate::model::geometry::primitives::AsAbstractCurveMut;
                &mut self.abstract_curve_mut().abstract_geometric_primitive
            }
        }
    };
}

impl_abstract_curve_traits!(AbstractCurve);

/// Discriminated union of all concrete curve implementations.
#[derive(Debug, Clone, PartialEq)]
pub enum CurveKind {
    /// A [`LineString`] curve.
    LineString(LineString),
}

impl AsAbstractCurve for CurveKind {
    fn abstract_curve(&self) -> &AbstractCurve {
        match self {
            Self::LineString(x) => x.abstract_curve(),
        }
    }
}

impl AsAbstractCurveMut for CurveKind {
    fn abstract_curve_mut(&mut self) -> &mut AbstractCurve {
        match self {
            Self::LineString(x) => x.abstract_curve_mut(),
        }
    }
}

impl_abstract_curve_traits!(CurveKind);

impl CurveKind {
    pub fn compute_envelope(&self) -> Envelope {
        match self {
            Self::LineString(x) => x.compute_envelope(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            Self::LineString(x) => x.apply_transform(m),
        }
    }
}
