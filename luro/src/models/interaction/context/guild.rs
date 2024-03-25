use luro_model::guild::Guild;

use crate::models::interaction::InteractionError;

impl super::InteractionContext {
    /// Return a guild if present
    pub async fn guild(&self) -> Result<Guild, InteractionError> {
        match self.interaction.guild_id {
            Some(guild_id) => Ok(self.gateway.database.fetch_guild(guild_id).await?),
            None => Err(InteractionError::NotGuild),
        }
    }
}
