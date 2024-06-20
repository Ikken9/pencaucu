use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Career {

    #[serde(rename = "careerName")]
    pub(crate) name: String
}

impl Display for Career {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}