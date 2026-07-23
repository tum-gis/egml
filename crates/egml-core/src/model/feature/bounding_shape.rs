use crate::model::basic_types::NilReason;
use crate::model::common::ApplyTransform;
use crate::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BoundingShape {
    envelope: Option<Envelope>,
    nil_reason: Option<NilReason>,
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

    pub fn envelope(&self) -> Option<&Envelope> {
        self.envelope.as_ref()
    }

    pub fn envelope_mut(&mut self) -> Option<&mut Envelope> {
        self.envelope.as_mut()
    }

    pub fn nil(nil_reason: NilReason) -> Self {
        Self {
            envelope: None,
            nil_reason: Some(nil_reason),
        }
    }
}

impl ApplyTransform for BoundingShape {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(envelope) = self.envelope.as_mut() {
            envelope.apply_transform(transform);
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(envelope) = self.envelope.as_mut() {
            envelope.apply_isometry(isometry);
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(envelope) = self.envelope.as_mut() {
            envelope.apply_translation(vector);
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(envelope) = self.envelope.as_mut() {
            envelope.apply_rotation(rotation);
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(envelope) = self.envelope.as_mut() {
            envelope.apply_scale(scale);
        }
    }
}
