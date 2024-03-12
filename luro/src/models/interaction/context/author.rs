use twilight_model::id::{marker::GuildMarker, Id};

use crate::models::{
    interaction::InteractionResult,
    MemberContext, User,
};

impl super::InteractionContext {
    pub async fn author(&self) -> InteractionResult<User> {
        self.gateway.database.fetch_user(self.author_id()?).await
    }

    pub async fn author_member(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> InteractionResult<MemberContext> {
        self.gateway
            .database
            .fetch_member(guild_id, self.author_id()?)
            .await
    }
}
