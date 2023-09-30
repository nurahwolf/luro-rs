use luro_model::ACCENT_COLOUR;
use tracing::warn;

use crate::{ComponentInteraction, Luro};

impl<T> ComponentInteraction<T> {
    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    pub async fn accent_colour(&self) -> u32 {
        match self.guild_id {
            Some(guild_id) => {
                match self
                    .get_guild(&guild_id)
                    .await
                    .map(|mut x| x.highest_role_colour().map(|x| x.0))
                {
                    Ok(colour) => colour.unwrap_or(ACCENT_COLOUR),
                    Err(why) => {
                        warn!(why = ?why, "Failed to get guild accent colour");
                        ACCENT_COLOUR
                    }
                }
            }
            None => ACCENT_COLOUR, // There is no guild for this interaction
        }
    }
}
