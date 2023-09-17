use luro_model::database_driver::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::MemberUpdate;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework {
    pub async fn member_update_listener(&self, event: Box<MemberUpdate>) -> anyhow::Result<()> {
        info!("Member {} updated in guild {}", event.user.id, event.guild_id);

        let mut user = self.database.get_user(&event.user.id).await?;
        user.update_member_update(event.clone());
        self.database.modify_user(&event.user.id, &user).await?;

        Ok(())
    }
}
