use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::TriangulatedSurface;
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{AbstractGeometryArrayProperty, AbstractGeometryProperty, Envelope};
use crate::{
    Error, impl_abstract_geometric_aggregate_mut_traits, impl_abstract_geometric_aggregate_traits,
    impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefMutIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiGeometry {
    pub abstract_geometric_aggregate: AbstractGeometricAggregate,
    geometry_members: Option<AbstractGeometryArrayProperty>,
    geometry_member: Vec<AbstractGeometryProperty>,
}

impl MultiGeometry {
    pub fn new(geometry_members: Option<AbstractGeometryArrayProperty>) -> Result<Self, Error> {
        Ok(Self {
            abstract_geometric_aggregate: AbstractGeometricAggregate::default(),
            geometry_members,
            geometry_member: Vec::new(),
        })
    }

    pub fn from_abstract_geometric_aggregate(
        abstract_geometric_aggregate: AbstractGeometricAggregate,
        geometry_members: Option<AbstractGeometryArrayProperty>,
    ) -> Self {
        Self {
            abstract_geometric_aggregate,
            geometry_members,
            geometry_member: Vec::new(),
        }
    }

    pub fn geometry_members(&self) -> Option<&AbstractGeometryArrayProperty> {
        self.geometry_members.as_ref()
    }

    pub fn set_geometry_members(&mut self, val: AbstractGeometryArrayProperty) {
        self.geometry_members = Some(val);
    }

    pub fn set_geometry_members_opt(&mut self, val: Option<AbstractGeometryArrayProperty>) {
        self.geometry_members = val;
    }

    pub fn clear_geometry_members(&mut self) {
        self.geometry_members = None;
    }

    pub fn geometry_member(&self) -> &[AbstractGeometryProperty] {
        &self.geometry_member
    }

    pub fn set_geometry_member(&mut self, val: Vec<AbstractGeometryProperty>) {
        self.geometry_member = val;
    }

    pub fn push_geometry_member(&mut self, member: AbstractGeometryProperty) {
        self.geometry_member.push(member);
    }

    pub fn extend_geometry_members(
        &mut self,
        members: impl IntoIterator<Item = AbstractGeometryProperty>,
    ) {
        self.geometry_member.extend(members);
    }
}

impl Triangulate for MultiGeometry {
    /// Triangulates all surface members and merges them into a single [`TriangulatedSurface`].
    ///
    /// Members that fail to triangulate individually (e.g. a degenerate ring) are
    /// skipped rather than failing the whole aggregate; see their errors via
    /// [`Triangulation::skipped`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if no member could be triangulated.
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let from_member = self.geometry_member.iter().flat_map(|x| x.object());
        let from_members = self
            .geometry_members
            .iter()
            .flat_map(|x| x.objects().iter());

        let mut surfaces = Vec::new();
        let mut skipped = Vec::new();

        for member in from_member.chain(from_members) {
            match member.triangulate() {
                Ok(triangulation) => {
                    let (surface, nested_skipped) = triangulation.into_parts();
                    surfaces.push(surface);
                    skipped.extend(nested_skipped);
                }
                Err(error) => {
                    skipped.push(error);
                }
            }
        }

        let combined = TriangulatedSurface::from_triangulated_surfaces(surfaces)?;
        Ok(Triangulation::new(combined, skipped))
    }
}

impl ApplyTransform for MultiGeometry {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(members) = &mut self.geometry_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_transform(transform));
        }
        self.geometry_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_transform(transform);
            }
        });
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(members) = &mut self.geometry_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_isometry(isometry));
        }
        self.geometry_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_isometry(isometry);
            }
        });
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(members) = &mut self.geometry_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_translation(vector));
        }
        self.geometry_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_translation(vector);
            }
        });
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(members) = &mut self.geometry_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_rotation(rotation));
        }
        self.geometry_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_rotation(rotation);
            }
        });
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(members) = &mut self.geometry_members {
            members
                .objects_mut()
                .par_iter_mut()
                .for_each(|p| p.apply_scale(scale));
        }
        self.geometry_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_scale(scale);
            }
        });
    }
}

impl ComputeEnvelope for MultiGeometry {
    fn compute_envelope(&self) -> Option<Envelope> {
        let from_member: Vec<Envelope> = self
            .geometry_member
            .iter()
            .flat_map(|x| x.object())
            .flat_map(|x| x.compute_envelope())
            .collect();

        let from_members: Vec<Envelope> = self
            .geometry_members
            .iter()
            .flat_map(|x| x.objects().iter())
            .flat_map(|x| x.compute_envelope())
            .collect();

        let envelopes: Vec<Envelope> = from_member.into_iter().chain(from_members).collect();
        Envelope::from_envelopes(&envelopes)
    }
}

impl AsAbstractGeometricAggregate for MultiGeometry {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        &self.abstract_geometric_aggregate
    }
}

impl AsAbstractGeometricAggregateMut for MultiGeometry {
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate {
        &mut self.abstract_geometric_aggregate
    }
}

impl_abstract_geometric_aggregate_traits!(MultiGeometry);
impl_abstract_geometric_aggregate_mut_traits!(MultiGeometry);
impl_has_geometry_type!(MultiGeometry, MultiGeometry);

impl IterGeometries for MultiGeometry {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(
                    self.geometry_members
                        .iter()
                        .flat_map(|members| members.objects().iter())
                        .flat_map(|x| x.iter_geometries()),
                )
                .chain(
                    self.geometry_member
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_geometries()),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::AbstractGeometryKind;
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractRingProperty,
        AbstractSurfaceKind, AsSurface, LinearRing, Polygon,
    };

    fn unit_square_polygon_kind(z: f64) -> AbstractGeometryKind {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, z).unwrap(),
            DirectPosition::new(1.0, 0.0, z).unwrap(),
            DirectPosition::new(1.0, 1.0, z).unwrap(),
            DirectPosition::new(0.0, 1.0, z).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring),
            )),
            vec![],
        )
        .unwrap();
        AbstractGeometryKind::AbstractGeometricPrimitiveKind(
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                polygon,
            )),
        )
    }

    #[test]
    fn triangulate_combines_geometry_member() {
        let mut multi_geometry = MultiGeometry::new(None).unwrap();
        multi_geometry.set_geometry_member(vec![
            AbstractGeometryProperty::from_object(unit_square_polygon_kind(0.0)),
            AbstractGeometryProperty::from_object(unit_square_polygon_kind(1.0)),
        ]);

        let triangulated = multi_geometry.triangulate().unwrap();

        assert_eq!(triangulated.surface().patches().objects_len(), 4);
    }

    #[test]
    fn triangulate_combines_geometry_members_array() {
        let geometry_members = AbstractGeometryArrayProperty::from_objects(vec![
            unit_square_polygon_kind(0.0),
            unit_square_polygon_kind(1.0),
        ]);
        let multi_geometry = MultiGeometry::new(Some(geometry_members)).unwrap();

        let triangulated = multi_geometry.triangulate().unwrap();

        assert_eq!(triangulated.surface().patches().objects_len(), 4);
    }

    #[test]
    fn triangulate_combines_both_member_and_members() {
        let geometry_members =
            AbstractGeometryArrayProperty::from_objects(vec![unit_square_polygon_kind(1.0)]);
        let mut multi_geometry = MultiGeometry::new(Some(geometry_members)).unwrap();
        multi_geometry.set_geometry_member(vec![AbstractGeometryProperty::from_object(
            unit_square_polygon_kind(0.0),
        )]);

        let triangulated = multi_geometry.triangulate().unwrap();

        assert_eq!(triangulated.surface().patches().objects_len(), 4);
    }
}
