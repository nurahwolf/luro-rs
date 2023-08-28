use std::time::{SystemTime, UNIX_EPOCH};

use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::{info, warn};
use twilight_model::gateway::payload::incoming::MemberRemove;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn member_remove_listener(&self, event: MemberRemove) -> anyhow::Result<()> {
        info!("Member {} removed / left from guild {}", event.user.id, event.guild_id);

        let mut user = self.database.get_user(&event.user.id).await?;
        match user.guilds.get_mut(&event.guild_id) {
            Some(member) => {
                let start = SystemTime::now();
                member.left_at = Some(start.duration_since(UNIX_EPOCH).expect("Time went backwards"));
                self.database.save_user(&event.user.id, &user).await?;
            }
            None => {
                warn!("No guild settings for user {} in guild {}", event.user.id, event.guild_id);
            }
        }

        Ok(())
    }
}
