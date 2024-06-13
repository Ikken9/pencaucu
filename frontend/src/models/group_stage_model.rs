use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GroupStageKey(i32);

// The result of the query fetcher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupStageData {
    id: u32,
}