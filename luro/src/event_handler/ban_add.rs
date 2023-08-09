use tracing::info;
use twilight_model::gateway::payload::incoming::BanAdd;

use crate::framework::Framework;
use luro_model::luro_database_driver::LuroDatabaseDriver;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn ban_add_listener(&self, ban: BanAdd) -> anyhow::Result<()> {
        info!("User {} banned from guild {}", ban.user.name, ban.guild_id);

        Ok(())
    }
}
