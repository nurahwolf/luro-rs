use luro_model::{
    database::Error,
    user::{MemberContext, User},
};
use twilight_model::id::{marker::GuildMarker, Id};

impl super::InteractionContext {
    pub async fn bot(&self) -> Result<User, Error> {
        self.gateway.database.fetch_user(self.gateway.current_user.id).await
    }

    pub async fn bot_member(&self, guild_id: Id<GuildMarker>) -> Result<MemberContext, Error> {
        self.gateway.database.fetch_member(guild_id, self.gateway.current_user.id).await
    }
}
