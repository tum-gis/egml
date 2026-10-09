use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::primitives::{
    AbstractRingKind, AbstractSurface, AsAbstractSurface, AsAbstractSurfaceMut,
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
    exterior: Option<AbstractRingKind>,
    interior: Vec<AbstractRingKind>,
}

impl Polygon {
    pub fn new(
        exterior: Option<AbstractRingKind>,
        interior: impl IntoIterator<Item = AbstractRingKind>,
    ) -> Result<Self, Error> {
        Ok(Self {
            abstract_surface: AbstractSurface::default(),
            exterior,
            interior: interior.into_iter().collect(),
        })
    }

    pub fn from_abstract_surface(
        abstract_surface: AbstractSurface,
        exterior: Option<AbstractRingKind>,
        interior: impl IntoIterator<Item = AbstractRingKind>,
    ) -> Self {
        Self {
            abstract_surface,
            exterior,
            interior: interior.into_iter().collect(),
        }
    }

    pub fn exterior(&self) -> Option<&AbstractRingKind> {
        self.exterior.as_ref()
    }

    pub fn set_exterior(&mut self, exterior: AbstractRingKind) {
        self.exterior = Some(exterior);
    }

    pub fn set_exterior_opt(&mut self, exterior: Option<AbstractRingKind>) {
        self.exterior = exterior;
    }

    pub fn clear_exterior(&mut self) {
        self.exterior = None;
    }

    pub fn interior(&self) -> &[AbstractRingKind] {
        &self.interior
    }

    pub fn set_interior(&mut self, interior: Vec<AbstractRingKind>) {
        self.interior = interior;
    }

    pub fn push_interior(&mut self, ring: AbstractRingKind) {
        self.interior.push(ring);
    }

    pub fn extend_interiors(&mut self, rings: impl IntoIterator<Item = AbstractRingKind>) {
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
        let mut enclosed_boundary_points =
            self.exterior().expect("should be there").points().to_vec();
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
    /// Returns [`Error::MissingExteriorRing`] if the polygon has no exterior ring.
    pub fn area_3d(&self) -> Result<f64, Error> {
        let exterior = self
            .exterior
            .as_ref()
            .ok_or(Error::MissingExteriorRing)?
            .area_3d();
        let holes = self.interior.iter().map(|r| r.area_3d()).sum::<f64>();

        Ok(exterior - holes)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        let mut all_points = Vec::new();
        if let Some(exterior) = &self.exterior {
            all_points.extend(exterior.points());
        }
        for ring in &self.interior {
            all_points.extend(ring.points());
        }

        all_points
    }
}

impl ApplyTransform for Polygon {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_transform(transform);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_transform(transform));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_isometry(isometry);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_translation(vector);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_rotation(rotation);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_scale(scale);
        }
        self.interior
            .par_iter_mut()
            .for_each(|p| p.apply_scale(scale));
    }
}

impl ComputeEnvelope for Polygon {
    fn compute_envelope(&self) -> Option<Envelope> {
        if let Some(exterior) = &self.exterior
            && let Some(e) = exterior.compute_envelope()
        {
            return Some(e);
        }

        let envelopes = self
            .interior
            .iter()
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
                        .into_iter()
                        .flat_map(|x| x.iter_geometries()),
                )
                .chain(self.interior.iter().flat_map(|x| x.iter_geometries())),
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
        let polygon = Polygon::new(Some(AbstractRingKind::LinearRing(ring)), []).unwrap();
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
            Some(AbstractRingKind::LinearRing(exterior)),
            vec![AbstractRingKind::LinearRing(hole)],
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
    fn triangulate_no_exterior_ring() {
        let polygon = Polygon::new(None, []).unwrap();
        assert_eq!(
            polygon.triangulate().err(),
            Some(Error::MissingExteriorRing)
        );
    }

    #[test]
    fn basic_normal_vector() {
        let point_a = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
        let point_b = DirectPosition::new(1.0, 0.0, 1.0).unwrap();
        let point_c = DirectPosition::new(1.0, 1.0, 1.0).unwrap();
        let point_d = DirectPosition::new(0.0, 1.0, 1.0).unwrap();
        let linear_ring = LinearRing::new([point_a, point_b, point_c, point_d]).unwrap();
        let linear_ring = AbstractRingKind::LinearRing(linear_ring);
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
        let linear_ring = AbstractRingKind::LinearRing(linear_ring);
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
        let linear_ring_exterior = AbstractRingKind::LinearRing(linear_ring_exterior);

        let polygon = Polygon::new(Some(linear_ring_exterior), vec![]).expect("should work");
        let triangulation = polygon.triangulate().expect("should work");
        assert_eq!(triangulation.surface().patches_len(), 2);
    }

    #[test]
    fn test_polygon_with_interior_triangulation() {
        // 2×2 rectangle in the tilted plane z = 2y (true size 2 × 2√5) with a
        // 1×1 hole (true size 1 × √5) — net area 4√5 − √5 = 3√5.
        let linear_ring_exterior = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).expect("should work"),
            DirectPosition::new(2.0, 0.0, 0.0).expect("should work"),
            DirectPosition::new(2.0, 2.0, 4.0).expect("should work"),
            DirectPosition::new(0.0, 2.0, 4.0).expect("should work"),
        ])
        .expect("should work");
        let linear_ring_interior = LinearRing::new([
            DirectPosition::new(0.5, 0.5, 1.0).expect("should work"),
            DirectPosition::new(0.5, 1.5, 3.0).expect("should work"),
            DirectPosition::new(1.5, 1.5, 3.0).expect("should work"),
            DirectPosition::new(1.5, 0.5, 1.0).expect("should work"),
        ])
        .expect("should work");

        let polygon = Polygon::new(
            Some(AbstractRingKind::LinearRing(linear_ring_exterior)),
            vec![AbstractRingKind::LinearRing(linear_ring_interior)],
        )
        .expect("should work");
        let triangulation = polygon.triangulate().expect("should work");

        let expected = 3.0 * 5.0_f64.sqrt();
        assert!(triangulation.skipped().is_empty());
        assert!((triangulation.surface().area_3d().expect("should work") - expected).abs() < 1e-10);
        assert!((polygon.area_3d().expect("should work") - expected).abs() < 1e-10);
    }
}
