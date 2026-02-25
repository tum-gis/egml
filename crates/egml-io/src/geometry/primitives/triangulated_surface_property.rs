use crate::primitives::GmlTriangulatedSurface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTriangulatedSurfaceProperty {
    #[serde(rename = "$value")]
    pub content: GmlTriangulatedSurface,
}
