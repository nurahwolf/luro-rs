use luro_model::database_driver::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::BanAdd;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework {
    pub async fn ban_add_listener(&self, ban: BanAdd) -> anyhow::Result<()> {
        info!("User {} ({}) banned from guild {}", ban.user.name, ban.user.id, ban.guild_id);

        Ok(())
    }
}
