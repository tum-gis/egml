use crate::primitives::GmlSolid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSolidProperty {
    #[serde(rename = "$value")]
    pub content: GmlSolid,
}
