use crate::model::geometry::primitives::{
    AbstractSurface, AsAbstractSurface, AsAbstractSurfaceMut, RingPropertyKind, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use crate::util::plane::Plane;
use crate::util::triangulate::triangulate;
use crate::{Error, impl_abstract_surface_traits};
use nalgebra::{Isometry3, Vector3};
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub(crate) abstract_surface: AbstractSurface,
    pub exterior: Option<RingPropertyKind>,
    pub interior: Vec<RingPropertyKind>,
}

impl Polygon {
    pub fn new(
        abstract_surface: AbstractSurface,
        exterior: Option<RingPropertyKind>,
        interior: Vec<RingPropertyKind>,
    ) -> Result<Self, Error> {
        Ok(Self {
            abstract_surface,
            exterior,
            interior,
        })
    }

    pub fn compute_envelope(&self) -> Envelope {
        Envelope::from_points(self.exterior.as_ref().expect("no linear ring").points())
            .expect("polygon must have valid points")
    }

    fn outer_boundary_points(&self) -> &[DirectPosition] {
        self.exterior.as_ref().expect("no linear ring").points()
    }

    fn outer_boundary_lower_corner(&self) -> DirectPosition {
        let x_min = self
            .outer_boundary_points()
            .iter()
            .min_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
            .unwrap()
            .x();
        let y_min = self
            .outer_boundary_points()
            .iter()
            .min_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
            .unwrap()
            .y();
        let z_min = self
            .outer_boundary_points()
            .iter()
            .min_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
            .unwrap()
            .z();

        DirectPosition::new(x_min, y_min, z_min).unwrap()
    }

    ///
    /// See also <https://www.khronos.org/opengl/wiki/Calculating_a_Surface_Normal#Newell.27s_Method>
    fn normal(&self) -> Vector3<f64> {
        let mut enclosed_boundary_points = self.outer_boundary_points().to_vec();
        enclosed_boundary_points.extend(self.outer_boundary_points().first());

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
        Plane::new(self.outer_boundary_lower_corner(), self.normal()).unwrap()
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        triangulate(self.exterior.clone(), self.interior.to_vec())
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        let mut all_points = Vec::new();
        if let Some(exterior) = &self.exterior {
            all_points.extend(exterior.points());
        }

        for ring in &self.interior {
            all_points.extend(ring.points().iter());
        }

        all_points
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(exterior) = &mut self.exterior {
            exterior.apply_transform(m);
        }

        self.interior.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::primitives::{AbstractRing, AsSurface, LinearRing};
    use nalgebra::Vector3;

    #[test]
    fn basic_normal_vector() {
        let point_a = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
        let point_b = DirectPosition::new(1.0, 0.0, 1.0).unwrap();
        let point_c = DirectPosition::new(1.0, 1.0, 1.0).unwrap();
        let point_d = DirectPosition::new(0.0, 1.0, 1.0).unwrap();
        let linear_ring = LinearRing::new(
            AbstractRing::default(),
            vec![point_a, point_b, point_c, point_d],
        )
        .unwrap();
        let linear_ring = RingPropertyKind::LinearRing(linear_ring);
        let polygon = Polygon::new(AbstractSurface::default(), Some(linear_ring), vec![]).unwrap();
        let normal = polygon.normal();

        assert_eq!(normal, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn basic_plane_equation() {
        let point_a = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
        let point_b = DirectPosition::new(1.0, 0.0, 1.0).unwrap();
        let point_c = DirectPosition::new(1.0, 1.0, 1.0).unwrap();
        let point_d = DirectPosition::new(0.0, 1.0, 1.0).unwrap();
        let linear_ring = LinearRing::new(
            AbstractRing::default(),
            vec![point_a, point_b, point_c, point_d],
        )
        .unwrap();
        let linear_ring = RingPropertyKind::LinearRing(linear_ring);
        let polygon = Polygon::new(AbstractSurface::default(), Some(linear_ring), vec![]).unwrap();
        let plane_equation = polygon.plane_equation();

        assert_eq!(
            plane_equation.point,
            DirectPosition::new(0.0, 0.0, 1.0).unwrap()
        );
        assert_eq!(plane_equation.normal(), Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_polygon_triangulation() {
        let linear_ring_exterior = LinearRing::new(
            AbstractRing::default(),
            vec![
                DirectPosition::new(0.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
                DirectPosition::new(0.0, 1.0, 2.0).expect("should work"),
            ],
        )
        .expect("should work");
        let linear_ring_exterior = RingPropertyKind::LinearRing(linear_ring_exterior);

        let polygon = Polygon::new(
            AbstractSurface::default(),
            Some(linear_ring_exterior),
            vec![],
        )
        .expect("should work");
        let triangulated_surface = polygon.triangulate().expect("should work");
        assert_eq!(triangulated_surface.patches_len(), 2);
    }

    #[test]
    fn test_polygon_with_interior_triangulation() {
        let linear_ring_exterior = LinearRing::new(
            AbstractRing::default(),
            vec![
                DirectPosition::new(0.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
                DirectPosition::new(0.0, 1.0, 2.0).expect("should work"),
                DirectPosition::new(0.0, 1.0, 3.0).expect("should work"),
                DirectPosition::new(0.0, 1.0, 5.0).expect("should work"),
            ],
        )
        .expect("should work");
        let linear_ring_exterior = RingPropertyKind::LinearRing(linear_ring_exterior);

        let linear_ring_interior = LinearRing::new(
            AbstractRing::default(),
            vec![
                DirectPosition::new(0.5, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
                DirectPosition::new(0.5, 1.0, 2.0).expect("should work"),
            ],
        )
        .expect("should work");
        let linear_ring_interior = RingPropertyKind::LinearRing(linear_ring_interior);

        let polygon = Polygon::new(
            AbstractSurface::default(),
            Some(linear_ring_exterior),
            vec![linear_ring_interior.clone(), linear_ring_interior.clone()],
        )
        .expect("should work");
        let triangulated_surface = polygon.triangulate().expect("should work");
        // assert_eq!(triangulated_surface.patches_len(), 2);
    }
}
