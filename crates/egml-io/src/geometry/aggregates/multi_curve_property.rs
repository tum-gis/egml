use crate::Error;
use crate::aggregates::GmlMultiCurve;
use egml_core::model::geometry::aggregates::MultiCurveProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiCurveProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(
        rename(serialize = "gml:MultiCurve", deserialize = "MultiCurve",),
        skip_serializing_if = "Option::is_none"
    )]
    pub object: Option<GmlMultiCurve>,
}

impl TryFrom<GmlMultiCurveProperty> for MultiCurveProperty {
    type Error = Error;

    fn try_from(item: GmlMultiCurveProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            href: item.href,
            object: item.object.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<&MultiCurveProperty> for GmlMultiCurveProperty {
    fn from(item: &MultiCurveProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}
