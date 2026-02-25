use crate::primitives::GmlPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointProperty {
    #[serde(rename = "$value")]
    pub point: GmlPoint,
}
