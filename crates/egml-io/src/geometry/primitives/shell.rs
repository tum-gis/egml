use crate::Error;
use crate::primitives::GmlSurfaceProperty;
use egml_core::model::geometry::primitives::{Shell, SurfaceProperty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlShell {
    #[serde(rename(serialize = "gml:surfaceMember", deserialize = "surfaceMember"))]
    pub(crate) members: Vec<GmlSurfaceProperty>,
}

impl From<&[SurfaceProperty]> for GmlShell {
    fn from(members: &[SurfaceProperty]) -> Self {
        Self {
            members: members.iter().map(GmlSurfaceProperty::from).collect(),
        }
    }
}

impl TryFrom<GmlShell> for Shell {
    type Error = Error;

    fn try_from(item: GmlShell) -> Result<Self, Self::Error> {
        Ok(Self::new(
            item.members
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        )?)
    }
}

impl From<&Shell> for GmlShell {
    fn from(item: &Shell) -> Self {
        Self {
            members: item
                .members()
                .iter()
                .map(GmlSurfaceProperty::from)
                .collect(),
        }
    }
}
