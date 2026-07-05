use crate::impl_abstract_ring_traits;
use crate::model::geometry::primitives::{
    AbstractRing, AsAbstractRing, AsAbstractRingMut, LinearRing,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

/// Discriminated union of all concrete ring implementations.
#[derive(Debug, Clone, PartialEq)]
pub enum RingKind {
    /// A [`LinearRing`] ring.
    LinearRing(LinearRing),

    RingKind(Box<RingKind>),
}

impl AsAbstractRing for RingKind {
    fn abstract_ring(&self) -> &AbstractRing {
        match self {
            Self::LinearRing(x) => x.abstract_ring(),
            Self::RingKind(x) => x.abstract_ring(),
        }
    }
}

impl AsAbstractRingMut for RingKind {
    fn abstract_ring_mut(&mut self) -> &mut AbstractRing {
        match self {
            Self::LinearRing(x) => x.abstract_ring_mut(),
            Self::RingKind(x) => x.abstract_ring_mut(),
        }
    }
}

impl_abstract_ring_traits!(RingKind);

impl RingKind {
    pub fn compute_envelope(&self) -> Envelope {
        match self {
            Self::LinearRing(x) => x.compute_envelope(),
            Self::RingKind(x) => x.compute_envelope(),
        }
    }

    pub fn points(&self) -> &[DirectPosition] {
        match self {
            Self::LinearRing(x) => x.points(),
            Self::RingKind(x) => x.points(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            Self::LinearRing(x) => x.apply_transform(m),
            Self::RingKind(x) => x.apply_transform(m),
        }
    }

    pub fn area_3d(&self) -> f64 {
        match self {
            Self::LinearRing(x) => x.area_3d(),
            Self::RingKind(x) => x.area_3d(),
        }
    }
}
