use crate::Error;
use crate::primitives::GmlCurveKind;
use egml_core::model::geometry::primitives::CurveProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCurveProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub object: Option<GmlCurveKind>,
}

impl TryFrom<GmlCurveProperty> for CurveProperty {
    type Error = Error;

    fn try_from(item: GmlCurveProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            href: item.href,
            object: item.object.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<&CurveProperty> for GmlCurveProperty {
    fn from(item: &CurveProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}
