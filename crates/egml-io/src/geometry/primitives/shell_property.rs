use crate::Error;
use crate::primitives::GmlShell;
use egml_core::model::geometry::primitives::ShellProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlShellProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(rename(serialize = "gml:Shell", deserialize = "Shell"))]
    pub(crate) object: Option<GmlShell>,
}

impl TryFrom<GmlShellProperty> for ShellProperty {
    type Error = Error;

    fn try_from(item: GmlShellProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            object: item.object.map(|x| x.try_into()).transpose()?,
            href: item.href,
        })
    }
}

impl From<&ShellProperty> for GmlShellProperty {
    fn from(item: &ShellProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}
