use crate::model::geometry::Envelope;
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::{PointArrayProperty, PointProperty};
use crate::{Error, impl_abstract_geometric_aggregate_traits};
use nalgebra::Isometry3;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiPoint {
    pub(crate) abstract_geometric_aggregate: AbstractGeometricAggregate,
    point_members: Option<PointArrayProperty>,
    point_member: Vec<PointProperty>,
}

impl MultiPoint {
    pub fn new(point_members: Option<PointArrayProperty>) -> Result<Self, Error> {
        Ok(Self {
            abstract_geometric_aggregate: AbstractGeometricAggregate::default(),
            point_members,
            point_member: Vec::new(),
        })
    }

    pub fn point_members(&self) -> Option<&PointArrayProperty> {
        self.point_members.as_ref()
    }

    pub fn set_point_members(&mut self, val: Option<PointArrayProperty>) {
        self.point_members = val;
    }

    pub fn point_member(&self) -> &[PointProperty] {
        &self.point_member
    }

    pub fn set_point_member(&mut self, val: Vec<PointProperty>) {
        self.point_member = val;
    }

    pub fn push_point_member(&mut self, member: PointProperty) {
        self.point_member.push(member);
    }

    pub fn extend_point_members(&mut self, members: impl IntoIterator<Item = PointProperty>) {
        self.point_member.extend(members);
    }
}

impl MultiPoint {
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(members) = &mut self.point_members {
            members
                .objects
                .par_iter_mut()
                .for_each(|p| p.apply_transform(m));
        }
        self.point_member.par_iter_mut().for_each(|p| {
            if let Some(object) = &mut p.object {
                object.apply_transform(m);
            }
        });
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        let points: Vec<_> = self
            .point_members
            .iter()
            .flat_map(|members| members.objects.iter())
            .chain(self.point_member.iter().flat_map(|x| x.object.as_ref()))
            .map(|p| *p.pos())
            .collect();
        Envelope::from_points(&points).ok()
    }
}

impl AsAbstractGeometricAggregate for MultiPoint {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        &self.abstract_geometric_aggregate
    }
}

impl AsAbstractGeometricAggregateMut for MultiPoint {
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate {
        &mut self.abstract_geometric_aggregate
    }
}

impl_abstract_geometric_aggregate_traits!(MultiPoint);
