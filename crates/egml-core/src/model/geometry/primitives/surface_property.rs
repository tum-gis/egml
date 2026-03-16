use crate::model::geometry::primitives::SurfaceKind;
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

/// An owned wrapper around a concrete [`SurfaceKind`].
///
/// Used as a property element in GML to hold an inline surface definition.
#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceProperty {
    // pub href: Option<String>,
    /// The surface value held by this property.
    pub content: SurfaceKind,
}

impl SurfaceProperty {
    /// Creates a new `SurfaceProperty` wrapping the given surface.
    pub fn new(content: SurfaceKind) -> Self {
        Self { content }
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
