use crate::Error;
use crate::model::geometry::primitives::SurfaceKind;
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceProperty {
    // pub href: Option<String>,
    pub content: SurfaceKind,
}

impl SurfaceProperty {
    pub fn new(content: SurfaceKind) -> Result<Self, Error> {
        Ok(Self { content })
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        self.content.points()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.content.apply_transform(m);
    }

    pub fn compute_envelope(&self) -> Envelope {
        self.content.compute_envelope()
    }
}
