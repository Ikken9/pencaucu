use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MatchKey(i32);

// The result of the query fetcher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchData {
    pub(crate) id: u32,
    pub(crate) date: String,
    pub(crate) admin_email: String
}