use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stadium {
    pub id: String,
    pub name: String,
    pub country: String,
    pub city: String
}