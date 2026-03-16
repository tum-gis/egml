use crate::model::geometry::Envelope;
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::CurveKind;
use crate::{Error, impl_abstract_geometric_aggregate_traits};
use nalgebra::Isometry3;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

/// An unordered collection of [`CurveKind`] members.
///
/// Corresponds to `gml:MultiCurve` in ISO 19136 §10.6.3.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiCurve {
    pub(crate) abstract_geometric_aggregate: AbstractGeometricAggregate,
    curve_member: Vec<CurveKind>,
}

impl MultiCurve {
    /// Creates a new `MultiCurve` from an ordered list of curve members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if `members` is empty.
    pub fn new(
        abstract_geometric_aggregate: AbstractGeometricAggregate,
        members: Vec<CurveKind>,
    ) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::EmptyCollection("multi curve"));
        }

        Ok(Self {
            abstract_geometric_aggregate,
            curve_member: members,
        })
    }

    /// Returns the curve members as a slice.
    pub fn curve_member(&self) -> &[CurveKind] {
        &self.curve_member
    }

    /// Replaces the curve members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if `val` is empty.
    pub fn set_curve_member(&mut self, val: Vec<CurveKind>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::EmptyCollection("multi curve"));
        }
        self.curve_member = val;
        Ok(())
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.curve_member.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }

    /// Returns the union of the bounding boxes of all curve members.
    pub fn compute_envelope(&self) -> Envelope {
        let envelopes: Vec<Envelope> = self
            .curve_member
            .iter()
            .map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
            .expect("MultiCurve must have at least one curve member")
    }
}

impl AsAbstractGeometricAggregate for MultiCurve {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        &self.abstract_geometric_aggregate
    }
}

impl AsAbstractGeometricAggregateMut for MultiCurve {
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate {
        &mut self.abstract_geometric_aggregate
    }
}

impl_abstract_geometric_aggregate_traits!(MultiCurve);
