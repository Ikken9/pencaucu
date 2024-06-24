use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub id: u64,
    pub date: i64,

    #[serde(rename = "knockoutStage")]
    pub knockout_stage: Option<String>,

    #[serde(rename = "stadiumName")]
    pub stadium_name: String,

    #[serde(rename = "teamName")]
    pub first_team_name: String,

    #[serde(rename = "facedTeamName")]
    pub second_team_name: String,

    #[serde(rename = "teamScore")]
    pub first_team_score: u32,

    #[serde(rename = "facedTeamScore")]
    pub second_team_score: u32,

    #[serde(rename = "teamPictureUrl")]
    pub team_picture_url: String,

    #[serde(rename = "facedTeamPictureUrl")]
    pub faced_team_picture_url: String,
}