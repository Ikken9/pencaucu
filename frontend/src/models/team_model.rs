use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TeamKey(i32);

// The result of the query fetcher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamData {
    name: String,
    url: String
}