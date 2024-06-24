use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchResult {

    #[serde(rename = "matchId")]
    pub match_id: u32,

    #[serde(rename = "teamScore")]
    pub first_team_score: u32,

    #[serde(rename = "facedTeamScore")]
    pub second_team_score: u32,

}