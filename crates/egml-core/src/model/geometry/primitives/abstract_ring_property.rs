use crate::model::geometry::primitives::{LinearRing, RingKind};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub enum RingPropertyKind {
    LinearRing(LinearRing),
    RingKind(RingKind),
}

impl RingPropertyKind {
    pub fn compute_envelope(&self) -> Envelope {
        match self {
            RingPropertyKind::LinearRing(ring) => ring.compute_envelope(),
            RingPropertyKind::RingKind(x) => x.compute_envelope(),
        }
    }

    pub fn points(&self) -> &[DirectPosition] {
        match self {
            RingPropertyKind::LinearRing(ring) => ring.points(),
            RingPropertyKind::RingKind(x) => x.points(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            RingPropertyKind::LinearRing(x) => x.apply_transform(m),
            RingPropertyKind::RingKind(x) => x.apply_transform(m),
        }
    }
}
