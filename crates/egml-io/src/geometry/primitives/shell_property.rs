use crate::primitives::GmlShell;
use egml_core::model::geometry::primitives::SurfaceProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlShellProperty {
    #[serde(rename(serialize = "gml:Shell", deserialize = "Shell"))]
    pub(crate) shell: Option<GmlShell>,
}

impl From<&[SurfaceProperty]> for GmlShellProperty {
    fn from(members: &[SurfaceProperty]) -> Self {
        Self {
            shell: Some(GmlShell::from(members)),
        }
    }
}
