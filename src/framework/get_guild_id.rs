use anyhow::anyhow;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::models::LuroResponse;

use super::LuroFramework;

impl LuroFramework {
    pub fn get_guild_id(&self, slash: &LuroResponse) -> anyhow::Result<Id<GuildMarker>> {
        match slash.interaction.guild_id {
            Some(guild_id) => Ok(guild_id),
            None => Err(anyhow!("No guild ID in this interactuin"))
        }
    }
}
