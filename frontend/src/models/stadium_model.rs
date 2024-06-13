use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct StadiumKey(i32);

// The result of the query fetcher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StadiumData {
    id: String,
    name: String,
    country: String,
    city: String
}