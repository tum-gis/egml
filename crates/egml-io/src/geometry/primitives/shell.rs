use crate::primitives::GmlSurfaceProperty;
use egml_core::model::geometry::primitives::SurfaceProperty;
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
