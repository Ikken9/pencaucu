use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KnockoutStage {
    pub(crate) id: u32,

    #[serde(rename = "stageName")]
    pub(crate) name: String
}