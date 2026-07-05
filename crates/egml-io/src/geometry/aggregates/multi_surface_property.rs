use crate::Error;
use crate::aggregates::GmlMultiSurface;
use egml_core::model::geometry::aggregates::MultiSurfaceProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiSurfaceProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(
        rename(serialize = "gml:MultiSurface", deserialize = "MultiSurface"),
        skip_serializing_if = "Option::is_none"
    )]
    pub object: Option<GmlMultiSurface>,
}

impl TryFrom<GmlMultiSurfaceProperty> for MultiSurfaceProperty {
    type Error = Error;

    fn try_from(item: GmlMultiSurfaceProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            href: item.href,
            object: item.object.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<&MultiSurfaceProperty> for GmlMultiSurfaceProperty {
    fn from(item: &MultiSurfaceProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}
