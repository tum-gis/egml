use crate::error::Error;
use nalgebra::Isometry3;

use crate::Error::ContainsEqualElements;
use crate::model::geometry::DirectPosition;
use crate::operations::geometry::Geometry;
use crate::operations::surface::Surface;
use parry3d_f64::query::PointQuery;

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
}

impl Geometry for Triangle {
    fn points(&self) -> Vec<&DirectPosition> {
        vec![&self.a, &self.b, &self.c]
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.a.apply_transform(m);
        self.b.apply_transform(m);
        self.c.apply_transform(m);
    }
}

impl Surface for Triangle {
    fn outer_boundary_points(&self) -> Vec<&DirectPosition> {
        vec![&self.a, &self.b, &self.c]
    }
}

impl From<Triangle> for parry3d_f64::shape::Triangle {
    fn from(item: Triangle) -> Self {
        Self::new(item.a.into(), item.b.into(), item.c.into())
    }
}
