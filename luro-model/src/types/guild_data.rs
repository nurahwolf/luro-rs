use serde::{Deserialize, Serialize};

/// Data that is only present when fetched from the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildData {
    pub accent_colour: Option<u32>,
    pub accent_colour_custom: Option<u32>,
}
