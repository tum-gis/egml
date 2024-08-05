use crate::model::geometry::DirectPosition;
use crate::operations::geometry::Geometry;
use crate::util::plane::Plane;
use nalgebra::Vector3;

pub trait Surface: Geometry {
    fn outer_boundary_points(&self) -> Vec<&DirectPosition>;

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
        let mut enclosed_boundary_points = self.outer_boundary_points();
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

    fn plane_equation(&self) -> Plane {
        Plane::new(self.outer_boundary_lower_corner(), self.normal()).unwrap()
    }
}
