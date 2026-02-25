use crate::primitives::GmlLinearRing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLinearRingProperty {
    #[serde(rename = "$value")]
    pub linear_ring: Option<GmlLinearRing>,
}
