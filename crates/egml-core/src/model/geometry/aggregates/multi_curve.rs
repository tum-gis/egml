use crate::model::geometry::Envelope;
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::CurveKind;
use crate::{Error, impl_abstract_geometric_aggregate_traits};
use nalgebra::Isometry3;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiCurve {
    pub(crate) abstract_geometric_aggregate: AbstractGeometricAggregate,
    curve_member: Vec<CurveKind>,
}

impl MultiCurve {
    pub fn new(
        abstract_geometric_aggregate: AbstractGeometricAggregate,
        members: Vec<CurveKind>,
    ) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::MustNotBeEmpty("multi surface"));
        }

        Ok(Self {
            abstract_geometric_aggregate,
            curve_member: members,
        })
    }

    pub fn curve_member(&self) -> &Vec<CurveKind> {
        self.curve_member.as_ref()
    }

    pub fn set_curve_member(&mut self, val: Vec<CurveKind>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::MustNotBeEmpty("multi curve"));
        }
        self.curve_member = val;
        Ok(())
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.curve_member.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }

    pub fn compute_envelope(&self) -> Envelope {
        let envelopes: Vec<Envelope> = self
            .curve_member
            .iter()
            .map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes.iter().collect::<Vec<_>>())
            .expect("MultiSurface must have at least one surface member")
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
