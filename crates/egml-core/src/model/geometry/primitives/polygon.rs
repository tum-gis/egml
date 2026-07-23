use crate::model::base::HasAssociationAttributes;
use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::primitives::{
    AbstractRingProperty, AbstractSurface, AsAbstractSurface, AsAbstractSurfaceMut,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::util::plane::Plane;
use crate::util::triangulate::triangulate;
use crate::{
    Error, impl_abstract_surface_mut_traits, impl_abstract_surface_traits, impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub abstract_surface: AbstractSurface,
    exterior: Option<AbstractRingProperty>,
    interior: Vec<AbstractRingProperty>,
}

impl Polygon {
    pub fn new(
        exterior: Option<AbstractRingProperty>,
        interior: impl IntoIterator<Item = AbstractRingProperty>,
    ) -> Result<Self, Error> {
        Ok(Self {
            abstract_surface: AbstractSurface::default(),
            exterior,
            interior: interior.into_iter().collect(),
        })
    }

    pub fn from_abstract_surface(
        abstract_surface: AbstractSurface,
        exterior: Option<AbstractRingProperty>,
        interior: impl IntoIterator<Item = AbstractRingProperty>,
    ) -> Self {
        Self {
            abstract_surface,
            exterior,
            interior: interior.into_iter().collect(),
        }
    }

    pub fn exterior(&self) -> Option<&AbstractRingProperty> {
        self.exterior.as_ref()
    }

    pub fn set_exterior(&mut self, exterior: AbstractRingProperty) {
        self.exterior = Some(exterior);
    }

    pub fn set_exterior_opt(&mut self, exterior: Option<AbstractRingProperty>) {
        self.exterior = exterior;
    }

    pub fn clear_exterior(&mut self) {
        self.exterior = None;
    }

    pub fn interior(&self) -> &[AbstractRingProperty] {
        &self.interior
    }

    pub fn set_interior(&mut self, interior: Vec<AbstractRingProperty>) {
        self.interior = interior;
    }

    pub fn push_interior(&mut self, ring: AbstractRingProperty) {
        self.interior.push(ring);
    }

    pub fn extend_interiors(&mut self, rings: impl IntoIterator<Item = AbstractRingProperty>) {
        self.interior.extend(rings);
    }
}

impl AsAbstractSurface for Polygon {
    fn abstract_surface(&self) -> &AbstractSurface {
        &self.abstract_surface
    }
}

impl AsAbstractSurfaceMut for Polygon {
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface {
        &mut self.abstract_surface
    }
}

impl_abstract_surface_traits!(Polygon);
impl_abstract_surface_mut_traits!(Polygon);
impl_has_geometry_type!(Polygon, Polygon);

impl Polygon {
    ///
    /// See also <https://www.khronos.org/opengl/wiki/Calculating_a_Surface_Normal#Newell.27s_Method>
    fn normal(&self) -> Vector3<f64> {
        let mut enclosed_boundary_points = self
            .exterior()
            .expect("should be there")
            .object()
            .expect("should be there")
            .points()
            .to_vec();
        let first = enclosed_boundary_points
            .first()
            .copied()
            .expect("should be there");
        enclosed_boundary_points.push(first);

        let mut normal = Vector3::new(0.0, 0.0, 0.0);
        for current_point_pair in enclosed_boundary_points.windows(2) {
            let current_first_point: Vector3<f64> = current_point_pair[0].into();
            let current_second_point: Vector3<f64> = current_point_pair[1].into();

            normal += (current_first_point - current_second_point)
                .cross(&(current_first_point + current_second_point));
        }

        normal.normalize()
    }

    pub fn plane_equation(&self) -> Plane {
        let envelope = self.compute_envelope().expect("should have envelope");
        Plane::new(*envelope.lower_corner(), self.normal())
    }

    /// Returns the net 3D area_3d of this polygon: exterior area_3d minus the sum of all interior hole area_3ds.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingExteriorRing`] if the polygon has no exterior ring property.
    /// Returns [`Error::UnresolvedRingReference`] if the exterior ring or any interior hole
    /// carries only an xlink:href that has not been resolved into an inline object.
    pub fn area_3d(&self) -> Result<f64, Error> {
        let exterior_ring = self.exterior.as_ref().ok_or(Error::MissingExteriorRing)?;
        let exterior = exterior_ring
            .object()
            .ok_or_else(|| Error::UnresolvedRingReference {
                href: exterior_ring.href().map(|h| h.to_string()),
            })?
            .area_3d();

        let holes = self
            .interior
            .iter()
            .map(|r| {
                r.object()
                    .ok_or_else(|| Error::UnresolvedRingReference {
                        href: r.href().map(|h| h.to_string()),
                    })
                    .map(|ring| ring.area_3d())
            })
            .collect::<Result<Vec<f64>, Error>>()?
            .into_iter()
            .sum::<f64>();

        Ok(exterior - holes)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        let mut all_points = Vec::new();
        if let Some(exterior) = &self.exterior
            && let Some(object) = exterior.object()
        {
            all_points.extend(object.points());
        }

        for ring in &self.interior {
            if let Some(object) = ring.object() {
                all_points.extend(object.points());
            }
        }

        all_points
    }
}

impl ApplyTransform for Polygon {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_transform(transform);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_transform(transform);
            }
        });
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_isometry(isometry);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_isometry(isometry);
            }
        });
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_translation(vector);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_translation(vector);
            }
        });
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_rotation(rotation);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_rotation(rotation);
            }
        });
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(exterior) = &mut self.exterior
            && let Some(object) = exterior.object_mut()
        {
            object.apply_scale(scale);
        }

        self.interior.par_iter_mut().for_each(|p| {
            if let Some(object) = p.object_mut() {
                object.apply_scale(scale);
            }
        });
    }
}

impl ComputeEnvelope for Polygon {
    fn compute_envelope(&self) -> Option<Envelope> {
        if let Some(exterior) = &self.exterior
            && let Some(object) = exterior.object()
            && let Some(e) = object.compute_envelope()
        {
            return Some(e);
        }

        let envelopes = self
            .interior
            .iter()
            .filter_map(|x| x.object())
            .filter_map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes)
    }
}

impl Triangulate for Polygon {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let surface = triangulate(self.exterior.clone(), self.interior.to_vec())?;
        Ok(Triangulation::new(surface, Vec::new()))
    }
}

impl IterGeometries for Polygon {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(
                    self.exterior
                        .as_ref()
                        .and_then(|x| x.object())
                        .into_iter()
                        .flat_map(|x| x.iter_geometries()),
                )
                .chain(
                    self.interior
                        .iter()
                        .filter_map(|x| x.object())
                        .flat_map(|x| x.iter_geometries()),
                ),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::primitives::{AbstractRingKind, AsSurface, LinearRing};
    use nalgebra::Vector3;

    #[test]
    fn area_3d_unit_square() {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 1.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 1.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 1.0).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring),
            )),
            [],
        )
        .unwrap();
        assert!((polygon.area_3d().expect("has exterior ring") - 1.0).abs() < 1e-10);
    }

    #[test]
    fn area_3d_with_hole() {
        // 4×4 outer square with a 1×1 hole — net area_3d should be 15.
        let exterior = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(4.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(4.0, 4.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 4.0, 0.0).unwrap(),
        ])
        .unwrap();
        let hole = LinearRing::new([
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(2.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(2.0, 2.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 2.0, 0.0).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(exterior),
            )),
            vec![AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(hole),
            )],
        )
        .unwrap();
        assert!((polygon.area_3d().expect("has exterior ring") - 15.0).abs() < 1e-10);
    }

    #[test]
    fn area_3d_no_exterior_ring() {
        let polygon = Polygon::new(None, []).unwrap();
        assert_eq!(polygon.area_3d(), Err(Error::MissingExteriorRing));
    }

    #[test]
    fn area_3d_unresolved_exterior_ring() {
        let exterior = AbstractRingProperty::from_href("urn:example:ring-1".into());
        let polygon = Polygon::new(Some(exterior), []).unwrap();
        assert_eq!(
            polygon.area_3d(),
            Err(Error::UnresolvedRingReference {
                href: Some("urn:example:ring-1".to_string())
            })
        );
    }

    #[test]
    fn area_3d_unresolved_interior_ring() {
        let exterior = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(4.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(4.0, 4.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 4.0, 0.0).unwrap(),
        ])
        .unwrap();
        let hole = AbstractRingProperty::from_href("urn:example:hole-1".into());
        let polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(exterior),
            )),
            vec![hole],
        )
        .unwrap();
        assert_eq!(
            polygon.area_3d(),
            Err(Error::UnresolvedRingReference {
                href: Some("urn:example:hole-1".to_string())
            })
        );
    }

    #[test]
    fn basic_normal_vector() {
        let point_a = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
        let point_b = DirectPosition::new(1.0, 0.0, 1.0).unwrap();
        let point_c = DirectPosition::new(1.0, 1.0, 1.0).unwrap();
        let point_d = DirectPosition::new(0.0, 1.0, 1.0).unwrap();
        let linear_ring = LinearRing::new([point_a, point_b, point_c, point_d]).unwrap();
        let linear_ring =
            AbstractRingProperty::from_object(AbstractRingKind::LinearRing(linear_ring));
        let polygon = Polygon::new(Some(linear_ring), []).unwrap();
        let normal = polygon.normal();

        assert_eq!(normal, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn basic_plane_equation() {
        let point_a = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
        let point_b = DirectPosition::new(1.0, 0.0, 1.0).unwrap();
        let point_c = DirectPosition::new(1.0, 1.0, 1.0).unwrap();
        let point_d = DirectPosition::new(0.0, 1.0, 1.0).unwrap();
        let linear_ring = LinearRing::new([point_a, point_b, point_c, point_d]).unwrap();
        let linear_ring =
            AbstractRingProperty::from_object(AbstractRingKind::LinearRing(linear_ring));
        let polygon = Polygon::new(Some(linear_ring), []).unwrap();
        let plane_equation = polygon.plane_equation();

        assert_eq!(
            plane_equation.point,
            DirectPosition::new(0.0, 0.0, 1.0).unwrap()
        );
        assert_eq!(plane_equation.normal(), Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_polygon_triangulation() {
        let linear_ring_exterior = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).expect("should work"),
            DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
            DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
            DirectPosition::new(0.0, 1.0, 2.0).expect("should work"),
        ])
        .expect("should work");
        let linear_ring_exterior =
            AbstractRingProperty::from_object(AbstractRingKind::LinearRing(linear_ring_exterior));

        let polygon = Polygon::new(Some(linear_ring_exterior), vec![]).expect("should work");
        let triangulation = polygon.triangulate().expect("should work");
        assert_eq!(triangulation.surface().patches_len(), 2);
    }

    #[test]
    fn test_polygon_with_interior_triangulation() {
        let linear_ring_exterior = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).expect("should work"),
            DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
            DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
            DirectPosition::new(0.0, 1.0, 2.0).expect("should work"),
            DirectPosition::new(0.0, 1.0, 3.0).expect("should work"),
            DirectPosition::new(0.0, 1.0, 5.0).expect("should work"),
        ])
        .expect("should work");
        let linear_ring_exterior =
            AbstractRingProperty::from_object(AbstractRingKind::LinearRing(linear_ring_exterior));

        let linear_ring_interior = LinearRing::new([
            DirectPosition::new(0.5, 0.0, 0.0).expect("should work"),
            DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
            DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
            DirectPosition::new(0.5, 1.0, 2.0).expect("should work"),
        ])
        .expect("should work");
        let linear_ring_interior =
            AbstractRingProperty::from_object(AbstractRingKind::LinearRing(linear_ring_interior));

        let polygon = Polygon::new(
            Some(linear_ring_exterior),
            vec![linear_ring_interior.clone(), linear_ring_interior.clone()],
        )
        .expect("should work");
        let triangulation = polygon.triangulate().expect("should work");
        // assert_eq!(triangulation.surface().patches_len(), 2);
    }
}
