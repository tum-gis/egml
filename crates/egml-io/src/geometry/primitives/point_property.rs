use crate::Error;
use crate::primitives::GmlPoint;
use egml_core::model::geometry::primitives::PointProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(rename(serialize = "gml:Point", deserialize = "Point"))]
    pub object: Option<GmlPoint>,
}

impl TryFrom<GmlPointProperty> for PointProperty {
    type Error = Error;

    fn try_from(item: GmlPointProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            href: item.href,
            object: item.object.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<&PointProperty> for GmlPointProperty {
    fn from(item: &PointProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}
