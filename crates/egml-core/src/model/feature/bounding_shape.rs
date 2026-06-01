use crate::model::basic::NilReason;
use crate::model::geometry::Envelope;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BoundingShape {
    pub envelope: Option<Envelope>,
    pub nil_reason: Option<NilReason>,
}

impl BoundingShape {
    pub fn new(envelope: Envelope) -> Self {
        Self {
            envelope: Some(envelope),
            nil_reason: None,
        }
    }

    pub fn new_unchecked(envelope: Option<Envelope>, nil_reason: Option<NilReason>) -> Self {
        Self {
            envelope,
            nil_reason,
        }
    }

    pub fn nil(nil_reason: NilReason) -> Self {
        Self {
            envelope: None,
            nil_reason: Some(nil_reason),
        }
    }
}
