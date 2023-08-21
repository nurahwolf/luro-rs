use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UserMarriages {
    /// When did they get married?
    pub timestamp: SystemTime,
    /// The user's optional reason for marrying
    #[serde(default)]
    pub reason: Option<String>,
    /// A randomly generated prposal text that was used to propose to them
    #[serde(default)]
    pub proposal: String
}
