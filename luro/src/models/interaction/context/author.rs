use luro_model::{database::Error, user::User};

impl super::InteractionContext {
    pub async fn author(&self) -> Result<User, Error> {
        self.gateway
            .database
            .fetch_member_or_user(self.interaction.guild_id, self.author_id())
            .await
    }
}
