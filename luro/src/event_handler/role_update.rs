use luro_model::database_driver::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::RoleUpdate;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework {
    pub async fn role_update_listener(&self, event: RoleUpdate) -> anyhow::Result<()> {
        info!("Role {} updated in guild {}", event.role.id, event.guild_id);

        let mut guild = self.database.get_guild(&event.guild_id).await?;
        guild.roles.insert(event.role.id, event.role.into());
        self.database.modify_guild(&event.guild_id, &guild).await?;

        Ok(())
    }
}
