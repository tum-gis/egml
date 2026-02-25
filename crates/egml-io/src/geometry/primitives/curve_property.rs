use crate::primitives::GmlCurveKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCurveProperty {
    #[serde(rename = "@href")]
    pub href: Option<String>,

    #[serde(rename = "$value")]
    pub content: Option<GmlCurveKind>,
}
