use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchResult {

    #[serde(rename = "matchId")]
    pub(crate) match_id: u32,

    #[serde(rename = "teamScore")]
    pub(crate) first_team_score: u32,

    #[serde(rename = "facedTeamScore")]
    pub(crate) second_team_score: u32,

}