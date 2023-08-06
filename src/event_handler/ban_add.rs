use tracing::info;
use twilight_model::gateway::payload::incoming::BanAdd;

use crate::framework::LuroFramework;

impl LuroFramework {
    pub async fn ban_add_listener(&self, ban: BanAdd) -> anyhow::Result<()> {
        info!("User {} banned from guild {}", ban.user.name, ban.guild_id);

        Ok(())
    }
}
