use crate::model::geometry::primitives::{
    AbstractCurve, AsAbstractCurve, AsAbstractCurveMut, LinearRing,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

/// Base data shared by all GML ring geometry types (ISO 19136 §10.5.3).
///
/// A ring is a closed curve used as the boundary of a surface patch.
/// The only concrete ring type currently implemented is [`LinearRing`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractRing {
    pub(crate) abstract_curve: AbstractCurve,
}

impl AbstractRing {
    pub fn new(abstract_curve: AbstractCurve) -> Self {
        Self { abstract_curve }
    }
}

/// Object-safe read accessor for [`AbstractRing`] fields.
pub trait AsAbstractRing: AsAbstractCurve {
    /// Returns a reference to the embedded [`AbstractRing`] base data.
    fn abstract_ring(&self) -> &AbstractRing;
}

/// Mutable companion to [`AsAbstractRing`].
pub trait AsAbstractRingMut: AsAbstractRing + AsAbstractCurveMut {
    /// Returns a mutable reference to the embedded [`AbstractRing`] base data.
    fn abstract_ring_mut(&mut self) -> &mut AbstractRing;
}

impl AsAbstractRing for AbstractRing {
    fn abstract_ring(&self) -> &AbstractRing {
        self
    }
}

impl AsAbstractRingMut for AbstractRing {
    fn abstract_ring_mut(&mut self) -> &mut AbstractRing {
        self
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_abstract_ring_traits {
    ($type:ty) => {
        $crate::impl_abstract_curve_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractCurve for $type {
            fn abstract_curve(&self) -> &$crate::model::geometry::primitives::AbstractCurve {
                use $crate::model::geometry::primitives::AsAbstractRing;
                &self.abstract_ring().abstract_curve
            }
        }

        impl $crate::model::geometry::primitives::AsAbstractCurveMut for $type {
            fn abstract_curve_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractCurve {
                use $crate::model::geometry::primitives::AsAbstractRingMut;
                &mut self.abstract_ring_mut().abstract_curve
            }
        }
    };
}

impl_abstract_ring_traits!(AbstractRing);

/// Discriminated union of all concrete ring implementations.
#[derive(Debug, Clone, PartialEq)]
pub enum RingKind {
    /// A [`LinearRing`] ring.
    LinearRing(LinearRing),
}

impl AsAbstractRing for RingKind {
    fn abstract_ring(&self) -> &AbstractRing {
        match self {
            Self::LinearRing(x) => x.abstract_ring(),
        }
    }
}

impl AsAbstractRingMut for RingKind {
    fn abstract_ring_mut(&mut self) -> &mut AbstractRing {
        match self {
            Self::LinearRing(x) => x.abstract_ring_mut(),
        }
    }
}

impl_abstract_ring_traits!(RingKind);

impl RingKind {
    pub fn compute_envelope(&self) -> Envelope {
        match self {
            Self::LinearRing(x) => x.compute_envelope(),
        }
    }

    pub fn points(&self) -> &[DirectPosition] {
        match self {
            Self::LinearRing(x) => x.points(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            Self::LinearRing(x) => x.apply_transform(m),
        }
    }
}
