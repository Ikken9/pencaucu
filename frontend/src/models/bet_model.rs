use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bet {

    #[serde(rename = "playerEmail")]
    pub(crate) player_email: String,

    #[serde(rename = "matchId")]
    pub(crate) match_id: u64,

    #[serde(rename = "teamScore")]
    pub(crate) team_score: u64,

    #[serde(rename = "facedTeamScore")]
    pub(crate) faced_team_score: u64,
}