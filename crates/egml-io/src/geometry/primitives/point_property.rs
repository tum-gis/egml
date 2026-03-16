use crate::primitives::GmlPoint;
use egml_core::model::geometry::primitives::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointProperty {
    #[serde(rename(serialize = "gml:Point", deserialize = "Point"))]
    pub point: GmlPoint,
}

impl From<&Point> for GmlPointProperty {
    fn from(point: &Point) -> Self {
        Self {
            point: GmlPoint::from(point),
        }
    }
}
