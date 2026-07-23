use crate::model::common::{ApplyTransform, ComputeEnvelope, IterGeometries};
use crate::model::geometry::Envelope;
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::{PointArrayProperty, PointProperty};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::{
    Error, impl_abstract_geometric_aggregate_mut_traits, impl_abstract_geometric_aggregate_traits,
    impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiPoint {
    pub abstract_geometric_aggregate: AbstractGeometricAggregate,
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

    pub fn from_abstract_geometric_aggregate(
        abstract_geometric_aggregate: AbstractGeometricAggregate,
        point_members: Option<PointArrayProperty>,
    ) -> Self {
        Self {
            abstract_geometric_aggregate,
            point_members,
            point_member: Vec::new(),
        }
    }

    pub fn point_members(&self) -> Option<&PointArrayProperty> {
        self.point_members.as_ref()
    }

    pub fn set_point_members(&mut self, val: PointArrayProperty) {
        self.point_members = Some(val);
    }

    pub fn set_point_members_opt(&mut self, val: Option<PointArrayProperty>) {
        self.point_members = val;
    }

    pub fn clear_point_members(&mut self) {
        self.point_members = None;
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

impl ApplyTransform for MultiPoint {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(members) = &mut self.point_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_transform(transform));
        }
        self.point_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_transform(transform);
            }
        });
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(members) = &mut self.point_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_isometry(isometry));
        }
        self.point_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_isometry(isometry);
            }
        });
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(members) = &mut self.point_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_translation(vector));
        }
        self.point_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_translation(vector);
            }
        });
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(members) = &mut self.point_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_rotation(rotation));
        }
        self.point_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_rotation(rotation);
            }
        });
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(members) = &mut self.point_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_scale(scale));
        }
        self.point_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_scale(scale);
            }
        });
    }
}

impl ComputeEnvelope for MultiPoint {
    fn compute_envelope(&self) -> Option<Envelope> {
        let points: Vec<_> = self
            .point_members
            .iter()
            .flat_map(|members| members.objects().iter())
            .chain(self.point_member.iter().flat_map(|x| x.object()))
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
impl_abstract_geometric_aggregate_mut_traits!(MultiPoint);
impl_has_geometry_type!(MultiPoint, MultiPoint);

impl IterGeometries for MultiPoint {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(
                    self.point_members
                        .iter()
                        .flat_map(|members| members.objects().iter())
                        .flat_map(|x| x.iter_geometries()),
                )
                .chain(
                    self.point_member
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_geometries()),
                ),
        )
    }
}
