use crate::Error;
use crate::primitives::line_string::GmlLineString;
use egml_core::model::geometry::primitives::CurveKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlCurveKind {
    #[serde(rename(serialize = "gml:LineString", deserialize = "LineString"))]
    LineString(GmlLineString),
}

impl TryFrom<GmlCurveKind> for CurveKind {
    type Error = Error;

    fn try_from(item: GmlCurveKind) -> Result<Self, Self::Error> {
        let curve_kind = match item {
            GmlCurveKind::LineString(x) => Self::LineString(x.try_into()?),
        };
        Ok(curve_kind)
    }
}

impl From<&CurveKind> for GmlCurveKind {
    fn from(kind: &CurveKind) -> Self {
        match kind {
            CurveKind::LineString(l) => GmlCurveKind::LineString(GmlLineString::from(l)),
        }
    }
}
