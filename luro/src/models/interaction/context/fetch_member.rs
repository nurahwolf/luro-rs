use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::models::{interaction::InteractionResult, MemberContext};

impl super::InteractionContext {
    /// Enforce that only member data is fetched
    pub async fn fetch_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> InteractionResult<MemberContext> {
        self.gateway.database.fetch_member(guild_id, user_id).await
    }
}
