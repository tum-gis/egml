use crate::error::Error;

use parry3d_f64::query::PointQuery;

use crate::Error::ContainsEqualElements;
use crate::{DirectPosition, Envelope};

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    /// First point of the triangle.
    pub a: DirectPosition,
    /// Second point of the triangle.
    pub b: DirectPosition,
    /// Third point of the triangle.
    pub c: DirectPosition,
}

impl Triangle {
    pub fn new(a: DirectPosition, b: DirectPosition, c: DirectPosition) -> Result<Self, Error> {
        if a == b || a == c || b == c {
            return Err(ContainsEqualElements);
        }
        /*let a_b_dist: f64 = distance(&a, &b);
        let b_c_dist: f64 = distance(&b, &c);
        let c_a_dist: f64 = distance(&c, &a);
        if a_b_dist <= f64::EPSILON || b_c_dist <= f64::EPSILON || c_a_dist <= f64::EPSILON {
            return Err(ContainsEqualElements);
        }*/

        Ok(Self { a, b, c })
    }

    pub fn points(&self) -> Vec<DirectPosition> {
        vec![self.a, self.b, self.c]
    }

    pub fn area(&self) -> f64 {
        let parry_triangle: parry3d_f64::shape::Triangle = self.clone().into();
        // parry_triangle.distance_to_local_point().
        parry_triangle.area()
    }

    pub fn distance_to_local_point(&self, p: &DirectPosition) -> f64 {
        let parry_triangle: parry3d_f64::shape::Triangle = self.clone().into();
        let point: nalgebra::Point3<f64> = (*p).into();
        parry_triangle.distance_to_local_point(&point, false)
    }

    // TODO: less duplication
    pub fn get_lower_corner(&self) -> DirectPosition {
        // let a = cmp::min(self.a.x(), self.b.x());
        let x_min = self
            .points()
            .iter()
            .min_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
            .unwrap()
            .x();
        let y_min = self
            .points()
            .iter()
            .min_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
            .unwrap()
            .y();
        let z_min = self
            .points()
            .iter()
            .min_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
            .unwrap()
            .z();

        DirectPosition::new(x_min, y_min, z_min).unwrap()
    }

    // TODO: less duplication
    pub fn get_upper_corner(&self) -> DirectPosition {
        let x_max = self
            .points()
            .iter()
            .max_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
            .unwrap()
            .x();
        let y_max = self
            .points()
            .iter()
            .max_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
            .unwrap()
            .y();
        let z_max = self
            .points()
            .iter()
            .max_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
            .unwrap()
            .z();

        DirectPosition::new(x_max, y_max, z_max).unwrap()
    }

    // TODO: less duplication
    pub fn get_envelope(&self) -> Result<Envelope, Error> {
        let lower = self.get_lower_corner();
        let upper = self.get_upper_corner();

        Envelope::new(lower, upper)
    }
}

impl From<Triangle> for parry3d_f64::shape::Triangle {
    fn from(item: Triangle) -> Self {
        Self::new(item.a.into(), item.b.into(), item.c.into())
    }
}
