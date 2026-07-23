use crate::model::geometry::Envelope;

pub trait ComputeEnvelope {
    fn compute_envelope(&self) -> Option<Envelope>;
}
