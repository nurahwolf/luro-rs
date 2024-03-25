use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::{
    database::twilight::{Database, Error},
    user::MemberContext,
};

impl Database {
    pub async fn fetch_member(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<MemberContext, Error> {
        let twilight_member = self.twilight_client.guild_member(guild_id, user_id).await?.model().await?;
        Ok((guild_id, twilight_member).into())
    }
}
