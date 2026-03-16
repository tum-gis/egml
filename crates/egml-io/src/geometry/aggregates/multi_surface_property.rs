use crate::aggregates::GmlMultiSurface;
use egml_core::model::geometry::aggregates::MultiSurface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiSurfaceProperty {
    #[serde(rename(serialize = "gml:MultiSurface", deserialize = "MultiSurface"))]
    pub content: GmlMultiSurface,
}

impl From<&MultiSurface> for GmlMultiSurfaceProperty {
    fn from(multi_surface: &MultiSurface) -> Self {
        Self {
            content: GmlMultiSurface::from(multi_surface),
        }
    }
}
