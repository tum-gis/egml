use crate::Error;
use crate::model::geometry::{DirectPosition, LinearRing};
use crate::operations::geometry::Geometry;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceProperty {
    pub href: String,
    pub linear_ring: Option<LinearRing>,
}

impl SurfaceProperty {
    pub fn new(href: String, linear_ring: Option<LinearRing>) -> Result<Self, Error> {
        Ok(Self { href, linear_ring })
    }
}

impl Geometry for SurfaceProperty {
    fn points(&self) -> Vec<&DirectPosition> {
        match &self.linear_ring {
            Some(ring) => ring.points(),
            None => Vec::new(),
        }
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(solid) = &mut self.linear_ring {
            solid.apply_transform(m);
        }
    }
}
