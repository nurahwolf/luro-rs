use sqlx::types::Json;
use sqlx::Error;
use twilight_model::util::ImageHash;

use crate::{DbMember, LuroDatabase};

impl LuroDatabase {
    pub async fn get_member(&self, user_id: i64, guild_id: i64) -> Result<Option<DbMember>, Error> {
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
                user_id = $1
                    and
                guild_id = $2",
            user_id,
            guild_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
