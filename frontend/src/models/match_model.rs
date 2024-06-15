use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub(crate) id: u32,
    pub(crate) date: String,
    pub(crate) first_team_name: String,
    pub(crate) second_team_name: String,
    pub(crate) first_team_score: String,
    pub(crate) second_team_score: String,
}