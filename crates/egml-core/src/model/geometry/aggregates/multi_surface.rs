use crate::error::Error;
use crate::impl_abstract_geometric_aggregate_traits;
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::SurfaceProperty;
use crate::model::geometry::primitives::TriangulatedSurface;
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;
use rayon::prelude::*;

/// An unordered collection of [`SurfaceKind`] members.
///
/// Corresponds to `gml:MultiSurface` in [OGC 07-036 §11.3.4.1](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq)]
pub struct MultiSurface {
    pub(crate) abstract_geometric_aggregate: AbstractGeometricAggregate,
    surface_member: Vec<SurfaceProperty>,
}

impl MultiSurface {
    /// Creates a new `MultiSurface` from a list of surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `members` is empty.
    pub fn new(members: impl IntoIterator<Item = SurfaceProperty>) -> Result<Self, Error> {
        let members: Vec<SurfaceProperty> = members.into_iter().collect();
        if members.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:MultiSurface",
                minimum: 1,
                spec: Some("OGC 07-036 §11.3.4.1"),
                id: None,
                detail: None,
            });
        }

        Ok(Self {
            abstract_geometric_aggregate: AbstractGeometricAggregate::default(),
            surface_member: members,
        })
    }

    /// Returns the surface members as a slice.
    pub fn surface_member(&self) -> &[SurfaceProperty] {
        &self.surface_member
    }

    /// Replaces the surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `val` is empty.
    pub fn set_surface_member(&mut self, val: Vec<SurfaceProperty>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:MultiSurface",
                minimum: 1,
                spec: Some("OGC 07-036 §11.3.4.1"),
                id: None,
                detail: None,
            });
        }
        self.surface_member = val;
        Ok(())
    }

    pub fn push_surface_member(&mut self, member: SurfaceProperty) {
        self.surface_member.push(member);
    }

    pub fn extend_surface_members(&mut self, members: impl IntoIterator<Item = SurfaceProperty>) {
        self.surface_member.extend(members);
    }
}

impl MultiSurface {
    /// Triangulates all surface members and merges them into a single [`TriangulatedSurface`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TriangulationFailed`] if any member cannot be triangulated.
    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surfaces: Vec<TriangulatedSurface> = self
            .surface_member
            .iter()
            .flat_map(|x| x.object.as_ref())
            .map(|x| x.triangulate())
            .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

        let combined_triangulated_surface =
            TriangulatedSurface::from_triangulated_surfaces(triangulated_surfaces)?;
        Ok(combined_triangulated_surface)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        self.surface_member
            .iter()
            .flat_map(|x| x.object.as_ref())
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
                s.object
                    .as_ref()
                    .ok_or_else(|| Error::UnresolvedSurfaceReference {
                        href: s.href.clone(),
                    })
                    .and_then(|kind| kind.area_3d())
            })
            .collect::<Result<Vec<f64>, Error>>()
            .map(|area_3ds| area_3ds.into_iter().sum())
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.surface_member.par_iter_mut().for_each(|p| {
            if let Some(x) = p.object.as_mut() {
                x.apply_transform(m);
            }
        });
    }

    /// Returns the union of the bounding boxes of all surface members.
    pub fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .surface_member
            .iter()
            .flat_map(|x| x.object.as_ref())
            .flat_map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::primitives::{LinearRing, RingKind, RingProperty, SurfaceKind};

    fn unit_square_polygon(z: f64) -> SurfaceKind {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, z).unwrap(),
            DirectPosition::new(1.0, 0.0, z).unwrap(),
            DirectPosition::new(1.0, 1.0, z).unwrap(),
            DirectPosition::new(0.0, 1.0, z).unwrap(),
        ])
        .unwrap();
        let polygon = crate::model::geometry::primitives::Polygon::new(
            Some(RingProperty::new(RingKind::LinearRing(ring))),
            vec![],
        )
        .unwrap();
        SurfaceKind::Polygon(polygon)
    }

    #[test]
    fn area_3d_two_unit_squares() {
        let multi_surface = MultiSurface::new([
            SurfaceProperty::new(unit_square_polygon(0.0)),
            SurfaceProperty::new(unit_square_polygon(1.0)),
        ])
        .unwrap();
        assert!((multi_surface.area_3d().unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn area_3d_unresolved_surface_reference() {
        let multi_surface = MultiSurface::new([SurfaceProperty::new_href(
            "urn:example:surface-1".to_string(),
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
