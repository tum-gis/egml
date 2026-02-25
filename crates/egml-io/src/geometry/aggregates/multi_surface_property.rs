use crate::aggregates::GmlMultiSurface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiSurfaceProperty {
    #[serde(rename = "$value")]
    pub content: GmlMultiSurface,
}
