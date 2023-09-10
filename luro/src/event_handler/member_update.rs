use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::MemberUpdate;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver,> Framework<D,> {
    pub async fn member_update_listener(&self, event: Box<MemberUpdate,>,) -> anyhow::Result<(),> {
        info!("Member {} updated in guild {}", event.user.id, event.guild_id);

        let mut user = self.database.get_user(&event.user.id, false,).await?;
        user.update_member_update(event.clone(),);
        self.database.save_user(&event.user.id, &user,).await?;

        Ok((),)
    }
}
