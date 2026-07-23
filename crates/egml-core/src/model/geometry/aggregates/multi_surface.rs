use crate::error::Error;
use crate::model::base::HasAssociationAttributes;
use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::AbstractSurfaceProperty;
use crate::model::geometry::primitives::TriangulatedSurface;
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{
    impl_abstract_geometric_aggregate_mut_traits, impl_abstract_geometric_aggregate_traits,
    impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::prelude::*;

/// An unordered collection of [`SurfaceKind`] members.
///
/// Corresponds to `gml:MultiSurface` in [OGC 07-036 §11.3.4.1](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq)]
pub struct MultiSurface {
    pub abstract_geometric_aggregate: AbstractGeometricAggregate,
    surface_member: Vec<AbstractSurfaceProperty>,
}

impl MultiSurface {
    /// Creates a new `MultiSurface` from a list of surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `members` is empty.
    pub fn new(members: impl IntoIterator<Item = AbstractSurfaceProperty>) -> Result<Self, Error> {
        let members: Vec<AbstractSurfaceProperty> = members.into_iter().collect();
        Self::validate(&members)?;

        Ok(Self {
            abstract_geometric_aggregate: AbstractGeometricAggregate::default(),
            surface_member: members,
        })
    }

    pub fn from_abstract_geometric_aggregate(
        abstract_geometric_aggregate: AbstractGeometricAggregate,
        members: impl IntoIterator<Item = AbstractSurfaceProperty>,
    ) -> Result<Self, Error> {
        let members: Vec<AbstractSurfaceProperty> = members.into_iter().collect();
        Self::validate(&members)?;

        Ok(Self {
            abstract_geometric_aggregate,
            surface_member: members,
        })
    }

    fn validate(members: &[AbstractSurfaceProperty]) -> Result<(), Error> {
        if members.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:MultiSurface",
                minimum: 1,
                spec: Some("OGC 07-036 §11.3.4.1"),
                id: None,
                detail: None,
            });
        }
        Ok(())
    }

    /// Returns the surface members as a slice.
    pub fn surface_member(&self) -> &[AbstractSurfaceProperty] {
        &self.surface_member
    }

    /// Replaces the surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `val` is empty.
    pub fn set_surface_member(&mut self, val: Vec<AbstractSurfaceProperty>) -> Result<(), Error> {
        Self::validate(&val)?;

        self.surface_member = val;
        Ok(())
    }

    pub fn push_surface_member(&mut self, member: AbstractSurfaceProperty) {
        self.surface_member.push(member);
    }

    pub fn extend_surface_members(
        &mut self,
        members: impl IntoIterator<Item = AbstractSurfaceProperty>,
    ) {
        self.surface_member.extend(members);
    }
}

impl AsAbstractGeometricAggregate for MultiSurface {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        &self.abstract_geometric_aggregate
    }
}

impl AsAbstractGeometricAggregateMut for MultiSurface {
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate {
        &mut self.abstract_geometric_aggregate
    }
}

impl_abstract_geometric_aggregate_traits!(MultiSurface);
impl_abstract_geometric_aggregate_mut_traits!(MultiSurface);
impl_has_geometry_type!(MultiSurface, MultiSurface);

impl MultiSurface {
    pub fn points(&self) -> Vec<&DirectPosition> {
        self.surface_member
            .iter()
            .flat_map(|x| x.object())
            .fold(Vec::new(), |mut acc, x| {
                acc.extend(x.points().iter());
                acc
            })
    }

    /// Returns the total 3D area_3d of all surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::UnresolvedSurfaceReference`] if any member carries only an
    /// xlink:href that has not been resolved into an inline object.
    /// Propagates any error from [`SurfaceKind::area_3d`].
    pub fn area_3d(&self) -> Result<f64, Error> {
        self.surface_member
            .iter()
            .map(|s| {
                s.object()
                    .ok_or_else(|| Error::UnresolvedSurfaceReference {
                        href: s.href().as_ref().map(|h| h.to_string()),
                    })
                    .and_then(|kind| kind.area_3d())
            })
            .collect::<Result<Vec<f64>, Error>>()
            .map(|area_3ds| area_3ds.into_iter().sum())
    }
}

impl Triangulate for MultiSurface {
    /// Members that fail to triangulate individually (e.g. a degenerate ring) are
    /// skipped rather than failing the whole aggregate; see their errors via
    /// [`Triangulation::skipped`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if no member could be triangulated.
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let mut surfaces = Vec::new();
        let mut skipped = Vec::new();

        for member in self.surface_member.iter().flat_map(|x| x.object()) {
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

impl ApplyTransform for MultiSurface {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.surface_member.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_transform(transform);
            }
        });
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.surface_member.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_isometry(isometry);
            }
        });
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.surface_member.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_translation(vector);
            }
        });
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.surface_member.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_rotation(rotation);
            }
        });
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.surface_member.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object_mut() {
                x.apply_scale(scale);
            }
        });
    }
}

impl ComputeEnvelope for MultiSurface {
    /// Returns the union of the bounding boxes of all surface members.
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .surface_member
            .iter()
            .flat_map(|x| x.object())
            .flat_map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

impl IterGeometries for MultiSurface {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(
                self.surface_member
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
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfaceKind, LinearRing,
    };

    fn unit_square_polygon(z: f64) -> AbstractSurfaceKind {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, z).unwrap(),
            DirectPosition::new(1.0, 0.0, z).unwrap(),
            DirectPosition::new(1.0, 1.0, z).unwrap(),
            DirectPosition::new(0.0, 1.0, z).unwrap(),
        ])
        .unwrap();
        let polygon = crate::model::geometry::primitives::Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring),
            )),
            vec![],
        )
        .unwrap();
        AbstractSurfaceKind::Polygon(polygon)
    }

    #[test]
    fn area_3d_two_unit_squares() {
        let multi_surface = MultiSurface::new([
            AbstractSurfaceProperty::from_object(unit_square_polygon(0.0)),
            AbstractSurfaceProperty::from_object(unit_square_polygon(1.0)),
        ])
        .unwrap();
        assert!((multi_surface.area_3d().unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn iter_geometries_yields_self_each_polygon_and_each_ring() {
        let multi_surface = MultiSurface::new([
            AbstractSurfaceProperty::from_object(unit_square_polygon(0.0)),
            AbstractSurfaceProperty::from_object(unit_square_polygon(1.0)),
        ])
        .unwrap();

        // self (1) + 2 polygons (2) + 2 rings, one per polygon (2) = 5
        assert_eq!(multi_surface.iter_geometries().count(), 5);
    }

    #[test]
    fn area_3d_unresolved_surface_reference() {
        let multi_surface = MultiSurface::new([AbstractSurfaceProperty::from_href(
            "urn:example:surface-1".into(),
        )])
        .unwrap();
        assert_eq!(
            multi_surface.area_3d(),
            Err(Error::UnresolvedSurfaceReference {
                href: Some("urn:example:surface-1".to_string())
            })
        );
    }
}
