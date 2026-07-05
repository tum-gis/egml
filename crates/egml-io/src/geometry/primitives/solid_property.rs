use crate::Error;
use crate::primitives::GmlSolid;
use egml_core::model::geometry::primitives::SolidProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSolidProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(
        rename(serialize = "gml:Solid", deserialize = "Solid"),
        skip_serializing_if = "Option::is_none"
    )]
    pub object: Option<GmlSolid>,
}

impl TryFrom<GmlSolidProperty> for SolidProperty {
    type Error = Error;

    fn try_from(item: GmlSolidProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            href: item.href,
            object: item.object.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<&SolidProperty> for GmlSolidProperty {
    fn from(item: &SolidProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}
