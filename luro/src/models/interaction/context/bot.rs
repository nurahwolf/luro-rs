use twilight_model::id::{marker::GuildMarker, Id};

use crate::models::{interaction::InteractionResult, MemberContext, User};

impl super::InteractionContext {
    pub async fn bot(&self) -> InteractionResult<User> {
        self.gateway
            .database
            .fetch_user(self.gateway.current_user.id)
            .await
    }

    pub async fn bot_member(&self, guild_id: Id<GuildMarker>) -> InteractionResult<MemberContext> {
        self.gateway
            .database
            .fetch_member(guild_id, self.gateway.current_user.id)
            .await
    }
}
