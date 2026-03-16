use crate::primitives::GmlTriangulatedSurface;
use egml_core::model::geometry::primitives::TriangulatedSurface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTriangulatedSurfaceProperty {
    #[serde(rename(
        serialize = "gml:TriangulatedSurface",
        deserialize = "TriangulatedSurface"
    ))]
    pub content: GmlTriangulatedSurface,
}

impl From<&TriangulatedSurface> for GmlTriangulatedSurfaceProperty {
    fn from(surface: &TriangulatedSurface) -> Self {
        Self {
            content: GmlTriangulatedSurface::from(surface),
        }
    }
}
