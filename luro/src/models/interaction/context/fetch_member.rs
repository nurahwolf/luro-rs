use luro_model::database::Error;
use luro_model::user::MemberContext;
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

impl super::InteractionContext {
    /// Enforce that only member data is fetched
    pub async fn fetch_member(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<MemberContext, Error> {
        self.gateway.database.fetch_member(guild_id, user_id).await
    }
}
