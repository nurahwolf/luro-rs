use luro_model::types::User;
use sqlx::Error;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::types::{DbGender, DbSexuality, DbUserPermissions};

impl crate::SQLxDriver {
    pub async fn get_members_of_guild(&self, guild_id: Id<GuildMarker>) -> Result<Vec<User>, Error> {
        let mut users = vec![];
        let result = sqlx::query_file!("queries/guild_fetch_members.sql", guild_id.get() as i64)
            .fetch_all(&self.pool)
            .await?;

        for user in result {
            if let Ok(Some(user)) = self.get_member(Id::new(user.user_id as u64), Id::new(user.guild_id as u64)).await {
                users.push(user)
            }
        }

        Ok(users)
    }
}
