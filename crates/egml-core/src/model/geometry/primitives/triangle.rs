use crate::Error::IdenticalPositions;
use crate::error::Error;
use crate::model::geometry::primitives::{
    AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;
use parry3d_f64::query::PointQuery;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub(crate) abstract_surface_patch: AbstractSurfacePatch,

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
            return Err(IdenticalPositions);
        }

        Ok(Self {
            abstract_surface_patch: AbstractSurfacePatch::default(),
            a,
            b,
            c,
        })
    }

    pub(crate) fn new_unchecked(a: DirectPosition, b: DirectPosition, c: DirectPosition) -> Self {
        Self {
            abstract_surface_patch: AbstractSurfacePatch::default(),
            a,
            b,
            c,
        }
    }

    pub fn distance_to_local_point(&self, p: &DirectPosition) -> f64 {
        let parry_triangle: parry3d_f64::shape::Triangle = self.clone().into();
        let point: parry3d_f64::math::Vector = (*p).into();
        parry_triangle.distance_to_local_point(point, false)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        vec![&self.a, &self.b, &self.c]
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.a.apply_transform(m);
        self.b.apply_transform(m);
        self.c.apply_transform(m);
    }

    pub fn compute_envelope(&self) -> Envelope {
        Envelope::from_points(&[self.a, self.b, self.c]).expect("triangle points are finite")
    }

    pub fn area(&self) -> f64 {
        let parry_triangle: parry3d_f64::shape::Triangle = self.clone().into();
        // parry_triangle.distance_to_local_point().
        parry_triangle.area()
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surface = TriangulatedSurface::from_triangles(vec![self.clone()])?;
        Ok(triangulated_surface)
    }
}

impl AsAbstractSurfacePatch for Triangle {
    fn abstract_surface_patch(&self) -> &AbstractSurfacePatch {
        &self.abstract_surface_patch
    }
}

impl AsAbstractSurfacePatchMut for Triangle {
    fn abstract_surface_patch_mut(&mut self) -> &mut AbstractSurfacePatch {
        &mut self.abstract_surface_patch
    }
}

impl From<Triangle> for parry3d_f64::shape::Triangle {
    fn from(item: Triangle) -> Self {
        Self::new(item.a.into(), item.b.into(), item.c.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_construction_test() {
        let triangle_result = Triangle::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
        );

        assert!(matches!(triangle_result, Err(Error::IdenticalPositions)));
    }

    #[test]
    fn triangle_distance_test() {
        let triangle = Triangle::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
        )
        .unwrap();

        let distance =
            triangle.distance_to_local_point(&DirectPosition::new(0.5, 0.5, 1.0).unwrap());

        assert_eq!(distance, 1.0);
    }

    #[test]
    fn triangle_construction_test_2() {
        let triangle_result = Triangle::new(
            DirectPosition::new(601.92791444745251, 1130.4631113024607, 9.0130903915382347)
                .unwrap(),
            DirectPosition::new(601.92791832847342, 1130.4631032795705, 9.0130907233102739)
                .unwrap(),
            DirectPosition::new(601.92791832847342, 1130.4631032795705, 9.0130907233102739)
                .unwrap(),
        );

        assert!(matches!(triangle_result, Err(Error::IdenticalPositions)));
    }
}
