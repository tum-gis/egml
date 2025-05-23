use crate::Error;
use crate::model::geometry::DirectPosition;
use nalgebra::Vector3;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Plane {
    pub point: DirectPosition,
    normal: Vector3<f64>,
}

impl Plane {
    pub fn new(point: DirectPosition, normal: Vector3<f64>) -> Result<Self, Error> {
        Ok(Self { point, normal })
    }

    pub fn normal(&self) -> Vector3<f64> {
        self.normal
    }
}
