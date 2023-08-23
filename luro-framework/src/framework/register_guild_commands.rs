use anyhow::anyhow;
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::{info, warn};
use twilight_model::id::{
    marker::{ApplicationMarker, GuildMarker},
    Id
};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Register the commands that are within the framework's global command cache
    pub async fn register_guild_commands(
        &self,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>
    ) -> anyhow::Result<()> {
        let client = self.interaction_client(application_id);
        let commands = match self.guild_commands.lock() {
            Ok(mutex) => match mutex.get(&guild_id) {
                Some(guild_commands) => guild_commands.clone(),
                None => {
                    warn!("Guild {guild_id} has no commands set. Clearing them...");
                    vec![]
                }
            },
            Err(why) => return Err(anyhow!("Failed to lock: {why}"))
        };

        client.set_guild_commands(guild_id, &commands).await?;
        info!("Successfully registered {} commands in guild {guild_id}!", commands.len());

        Ok(())
    }
}
