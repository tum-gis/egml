use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlRing {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    // TODO: needs to be implemented
}
