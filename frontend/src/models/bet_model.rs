use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bet {

    #[serde(rename = "playerEmail")]
    pub player_email: String,

    #[serde(rename = "matchId")]
    pub match_id: u64,

    #[serde(rename = "teamScore")]
    pub team_score: u64,

    #[serde(rename = "facedTeamScore")]
    pub faced_team_score: u64,
}