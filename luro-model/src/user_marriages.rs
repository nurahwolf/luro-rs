use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserMarriages {
    pub timestamp: SystemTime,
    pub user: Id<UserMarker>,
    pub reason: String
}
