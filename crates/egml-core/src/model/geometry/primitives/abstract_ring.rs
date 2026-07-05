use crate::model::geometry::primitives::{AbstractCurve, AsAbstractCurve, AsAbstractCurveMut};
/// Base data shared by all GML ring geometry types ([OGC 07-036 §10.5.6](https://docs.ogc.org/is/07-036/07-036.pdf)).
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
