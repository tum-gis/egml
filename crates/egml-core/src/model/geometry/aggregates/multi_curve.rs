use crate::model::base::HasAssociationAttributes;
use crate::model::common::{ApplyTransform, ComputeEnvelope, IterGeometries};
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::AbstractCurveProperty;
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{
    Error, impl_abstract_geometric_aggregate_mut_traits, impl_abstract_geometric_aggregate_traits,
    impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

/// An unordered collection of [`AbstractCurveKind`](crate::model::geometry::primitives::AbstractCurveKind) members.
///
/// Corresponds to `gml:MultiCurve` in [OGC 07-036 §11.3.3.1](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq)]
pub struct MultiCurve {
    pub abstract_geometric_aggregate: AbstractGeometricAggregate,
    curve_member: Vec<AbstractCurveProperty>,
}

impl MultiCurve {
    /// Creates a new `MultiCurve` from an ordered list of curve members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `members` is empty.
    pub fn new(members: impl IntoIterator<Item = AbstractCurveProperty>) -> Result<Self, Error> {
        let members: Vec<AbstractCurveProperty> = members.into_iter().collect();
        Self::validate(&members)?;

        Ok(Self {
            abstract_geometric_aggregate: AbstractGeometricAggregate::default(),
            curve_member: members,
        })
    }

    pub fn from_abstract_geometric_aggregate(
        abstract_geometric_aggregate: AbstractGeometricAggregate,
        members: impl IntoIterator<Item = AbstractCurveProperty>,
    ) -> Result<Self, Error> {
        let members: Vec<AbstractCurveProperty> = members.into_iter().collect();
        Self::validate(&members)?;

        Ok(Self {
            abstract_geometric_aggregate,
            curve_member: members,
        })
    }

    fn validate(members: &[AbstractCurveProperty]) -> Result<(), Error> {
        if members.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:MultiCurve",
                minimum: 1,
                spec: Some("OGC 07-036 §11.3.3.1"),
                id: None,
                detail: None,
            });
        }
        Ok(())
    }

    /// Returns the curve members as a slice.
    pub fn curve_member(&self) -> &[AbstractCurveProperty] {
        &self.curve_member
    }

    /// Replaces the curve members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `val` is empty.
    pub fn set_curve_member(&mut self, val: Vec<AbstractCurveProperty>) -> Result<(), Error> {
        Self::validate(&val)?;
        self.curve_member = val;
        Ok(())
    }

    pub fn push_curve_member(&mut self, member: AbstractCurveProperty) {
        self.curve_member.push(member);
    }

    pub fn extend_curve_members(
        &mut self,
        members: impl IntoIterator<Item = AbstractCurveProperty>,
    ) {
        self.curve_member.extend(members);
    }
}

impl MultiCurve {
    /// Returns the positions of all resolved curve members, in member order.
    ///
    /// Members that carry only an unresolved xlink:href are skipped.
    pub fn points(&self) -> Vec<&DirectPosition> {
        self.curve_member
            .iter()
            .flat_map(|x| x.object())
            .flat_map(|x| x.points())
            .collect()
    }

    /// Returns the total 3D length of all curve members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::UnresolvedCurveReference`] if any member carries only an
    /// xlink:href that has not been resolved into an inline object.
    pub fn length_3d(&self) -> Result<f64, Error> {
        self.curve_member
            .iter()
            .map(|c| {
                c.object()
                    .ok_or_else(|| Error::UnresolvedCurveReference {
                        href: c.href().as_ref().map(|h| h.to_string()),
                    })
                    .map(|curve| curve.length_3d())
            })
            .collect::<Result<Vec<f64>, Error>>()
            .map(|lengths| lengths.into_iter().sum())
    }

    /// Returns the minimum 2D distance between `point` and any curve member,
    /// measured in the XY plane (z coordinates are ignored).
    ///
    /// # Errors
    ///
    /// Returns [`Error::UnresolvedCurveReference`] if any member carries only an
    /// xlink:href that has not been resolved into an inline object.
    pub fn distance_2d(&self, point: &DirectPosition) -> Result<f64, Error> {
        self.curve_member.iter().try_fold(f64::INFINITY, |min, c| {
            let curve = c.object().ok_or_else(|| Error::UnresolvedCurveReference {
                href: c.href().as_ref().map(|h| h.to_string()),
            })?;
            Ok(min.min(curve.distance_2d(point)))
        })
    }
}

impl ApplyTransform for MultiCurve {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.curve_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_transform(transform);
            }
        });
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.curve_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_isometry(isometry);
            }
        });
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.curve_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_translation(vector);
            }
        });
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.curve_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_rotation(rotation);
            }
        });
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.curve_member.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_scale(scale);
            }
        });
    }
}

impl ComputeEnvelope for MultiCurve {
    /// Returns the union of the bounding boxes of all curve members.
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .curve_member
            .iter()
            .flat_map(|x| x.object())
            .filter_map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
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
impl_abstract_geometric_aggregate_mut_traits!(MultiCurve);
impl_has_geometry_type!(MultiCurve, MultiCurve);

impl IterGeometries for MultiCurve {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(
                self.curve_member
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
    use crate::Error;
    use crate::model::geometry::primitives::{
        AbstractCurveKind, AbstractRingKind, LineString, LinearRing,
    };

    fn line_string(points: Vec<DirectPosition>) -> AbstractCurveProperty {
        AbstractCurveProperty::from_object(AbstractCurveKind::LineString(
            LineString::new(points).unwrap(),
        ))
    }

    #[test]
    fn length_3d_two_segments() {
        // Two unit segments along X — total length 2.
        let multi_curve = MultiCurve::new([
            line_string(vec![
                DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            ]),
            line_string(vec![
                DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(2.0, 0.0, 0.0).unwrap(),
            ]),
        ])
        .unwrap();
        assert!((multi_curve.length_3d().unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn length_3d_with_ring_member() {
        // Unit-square ring (perimeter 4) plus a unit segment — total length 5.
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let multi_curve = MultiCurve::new([
            AbstractCurveProperty::from_object(AbstractCurveKind::AbstractRingKind(
                AbstractRingKind::LinearRing(ring),
            )),
            line_string(vec![
                DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(0.0, 0.0, 1.0).unwrap(),
            ]),
        ])
        .unwrap();
        assert!((multi_curve.length_3d().unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn length_3d_unresolved_curve_reference() {
        let multi_curve = MultiCurve::new([AbstractCurveProperty::from_href(
            "urn:example:curve-1".into(),
        )])
        .unwrap();
        assert_eq!(
            multi_curve.length_3d(),
            Err(Error::UnresolvedCurveReference {
                href: Some("urn:example:curve-1".to_string())
            })
        );
    }

    fn pos(x: f64, y: f64, z: f64) -> DirectPosition {
        DirectPosition::new(x, y, z).unwrap()
    }

    #[test]
    fn distance_2d_takes_minimum_over_members() {
        // Two parallel lines at y=0 and y=10; the point is closer to the second.
        let multi_curve = MultiCurve::new([
            line_string(vec![pos(0.0, 0.0, 0.0), pos(10.0, 0.0, 0.0)]),
            line_string(vec![pos(0.0, 10.0, 0.0), pos(10.0, 10.0, 0.0)]),
        ])
        .unwrap();
        let d = multi_curve.distance_2d(&pos(5.0, 7.0, 0.0)).unwrap();
        assert!((d - 3.0).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_with_ring_member() {
        let ring = LinearRing::new([
            pos(20.0, 0.0, 0.0),
            pos(30.0, 0.0, 0.0),
            pos(30.0, 10.0, 0.0),
            pos(20.0, 10.0, 0.0),
        ])
        .unwrap();
        let multi_curve = MultiCurve::new([
            line_string(vec![pos(0.0, 0.0, 0.0), pos(0.0, 10.0, 0.0)]),
            AbstractCurveProperty::from_object(AbstractCurveKind::AbstractRingKind(
                AbstractRingKind::LinearRing(ring),
            )),
        ])
        .unwrap();
        // Inside the ring: 1 to its left edge (x=20), 21 to the line at x=0.
        let d = multi_curve.distance_2d(&pos(21.0, 5.0, 0.0)).unwrap();
        assert!((d - 1.0).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_ignores_z() {
        let multi_curve = MultiCurve::new([line_string(vec![
            pos(0.0, 0.0, -10.0),
            pos(10.0, 0.0, 10.0),
        ])])
        .unwrap();
        let d = multi_curve.distance_2d(&pos(5.0, 4.0, 500.0)).unwrap();
        assert!((d - 4.0).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_unresolved_curve_reference() {
        let multi_curve = MultiCurve::new([
            line_string(vec![pos(0.0, 0.0, 0.0), pos(1.0, 0.0, 0.0)]),
            AbstractCurveProperty::from_href("urn:example:curve-1".into()),
        ])
        .unwrap();
        assert_eq!(
            multi_curve.distance_2d(&pos(0.0, 0.0, 0.0)),
            Err(Error::UnresolvedCurveReference {
                href: Some("urn:example:curve-1".to_string())
            })
        );
    }
}
