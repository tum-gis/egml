use crate::model::common::ComputeEnvelope;
use crate::model::feature::AsAbstractFeatureMut;

pub trait RecomputeBoundingShape {
    fn recompute_bounding_shape(&mut self);
}

impl<T> RecomputeBoundingShape for T
where
    T: ComputeEnvelope + AsAbstractFeatureMut,
{
    fn recompute_bounding_shape(&mut self) {
        let Some(recomputed_envelope) = self.compute_envelope() else {
            return;
        };

        if let Some(bounding_shape) = self.bounded_by_mut() {
            if let Some(envelope) = bounding_shape.envelope_mut() {
                envelope.set_lower_corner(*recomputed_envelope.lower_corner());
                envelope.set_upper_corner(*recomputed_envelope.upper_corner());
            }
        } else {
            AsAbstractFeatureMut::set_bounding_shape_from_envelope(self, self.compute_envelope());
        }
    }
}
