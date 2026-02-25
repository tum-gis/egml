use crate::aggregates::GmlMultiCurve;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiCurveProperty {
    #[serde(rename = "$value")]
    pub content: GmlMultiCurve,
}
