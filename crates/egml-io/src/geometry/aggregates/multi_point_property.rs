use crate::Error;
use crate::aggregates::GmlMultiPoint;
use egml_core::model::geometry::aggregates::MultiPointProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiPointProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(
        rename(serialize = "gml:MultiPoint", deserialize = "MultiPoint",),
        skip_serializing_if = "Option::is_none"
    )]
    pub object: Option<GmlMultiPoint>,
}

impl TryFrom<GmlMultiPointProperty> for MultiPointProperty {
    type Error = Error;

    fn try_from(item: GmlMultiPointProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            href: item.href,
            object: item.object.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<&MultiPointProperty> for GmlMultiPointProperty {
    fn from(item: &MultiPointProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}
