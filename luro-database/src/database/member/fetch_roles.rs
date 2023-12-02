use std::collections::HashMap;

use twilight_model::id::{
    marker::{GuildMarker, RoleMarker, UserMarker},
    Id,
};

impl crate::Database {
    pub async fn member_fetch_roles(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> anyhow::Result<HashMap<Id<RoleMarker>, luro_model::types::Role>> {
        self.driver.member_fetch_roles(guild_id, user_id).await
    }
}
