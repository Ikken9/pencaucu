use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stadium {
    id: String,
    name: String,
    country: String,
    city: String
}