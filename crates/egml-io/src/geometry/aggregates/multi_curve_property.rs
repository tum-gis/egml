use crate::aggregates::GmlMultiCurve;
use egml_core::model::geometry::aggregates::MultiCurve;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiCurveProperty {
    #[serde(rename(serialize = "gml:MultiCurve", deserialize = "MultiCurve"))]
    pub content: GmlMultiCurve,
}

impl From<&MultiCurve> for GmlMultiCurveProperty {
    fn from(multi_curve: &MultiCurve) -> Self {
        Self {
            content: GmlMultiCurve::from(multi_curve),
        }
    }
}
