use sqlx::types::Json;
use sqlx::Error;
use twilight_model::util::ImageHash;

use crate::{DbMember, LuroDatabase};

impl LuroDatabase {
    pub async fn get_members_of_guild(&self, guild_id: i64) -> Result<Vec<DbMember>, Error> {
        sqlx::query_as!(
            DbMember,
            "SELECT
                user_id,
                guild_id,            
                avatar as \"avatar: Json<ImageHash>\",
                boosting_since,
                communication_disabled_until,
                deafened,
                flags,
                muted,
                nickname,
                pending
            FROM
                guild_members
            WHERE
                guild_id = $1",
            guild_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_guilds_of_member(&self, user_id: i64) -> Result<Vec<DbMember>, Error> {
        sqlx::query_as!(
            DbMember,
            "SELECT
                user_id,
                guild_id,            
                avatar as \"avatar: Json<ImageHash>\",
                boosting_since,
                communication_disabled_until,
                deafened,
                flags,
                muted,
                nickname,
                pending
            FROM
                guild_members
            WHERE
                user_id = $1",
            user_id,
        )
        .fetch_all(&self.pool)
        .await
    }
}
