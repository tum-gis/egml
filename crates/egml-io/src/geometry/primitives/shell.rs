use crate::primitives::GmlSurfaceProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlShell {
    #[serde(rename = "$value")]
    pub(crate) members: Vec<GmlSurfaceProperty>,
}
