use crate::primitives::GmlShell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlShellProperty {
    #[serde(rename = "$value")]
    pub(crate) shell: Option<GmlShell>,
}
