use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::MemberAdd;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver,> Framework<D,> {
    pub async fn member_add_listener(&self, event: Box<MemberAdd,>,) -> anyhow::Result<(),> {
        info!("Member {} joined guild {}", event.user.id, event.guild_id);

        let mut user = self.database.get_user(&event.user.id, false,).await?;
        user.update_member_add(event.clone(),);
        self.database.save_user(&event.user.id, &user,).await?;

        Ok((),)
    }
}
