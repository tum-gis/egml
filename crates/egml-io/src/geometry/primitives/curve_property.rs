use crate::primitives::GmlCurveKind;
use egml_core::model::geometry::primitives::CurveKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCurveProperty {
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub content: Option<GmlCurveKind>,
}

impl From<&CurveKind> for GmlCurveProperty {
    fn from(kind: &CurveKind) -> Self {
        Self {
            href: None,
            content: Some(GmlCurveKind::from(kind)),
        }
    }
}
