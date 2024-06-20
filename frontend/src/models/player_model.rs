use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub(crate) username: String,

    #[serde(rename = "profilePicture")]
    pub(crate) profile_picture: Option<String>,

    pub(crate) points: u64
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.username, self.points)
    }
}