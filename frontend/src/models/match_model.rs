use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub(crate) id: u64,
    pub(crate) date: i64,

    #[serde(rename = "knockoutStage")]
    pub(crate) knockout_stage: Option<String>,

    #[serde(rename = "stadiumName")]
    pub(crate) stadium_name: String,

    #[serde(rename = "teamName")]
    pub(crate) first_team_name: String,

    #[serde(rename = "facedTeamName")]
    pub(crate) second_team_name: String,

    #[serde(rename = "teamScore")]
    pub(crate) first_team_score: u32,

    #[serde(rename = "facedTeamScore")]
    pub(crate) second_team_score: u32,

    #[serde(rename = "teamPictureUrl")]
    pub(crate) team_picture_url: String,

    #[serde(rename = "facedTeamPictureUrl")]
    pub(crate) faced_team_picture_url: String,
}