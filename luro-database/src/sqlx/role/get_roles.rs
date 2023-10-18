use twilight_model::guild::RoleTags;
use twilight_model::util::ImageHash;
use sqlx::types::Json;
use crate::{LuroDatabase, DbRole};

impl LuroDatabase {
    pub async fn get_guild_roles(&self, guild_id: i64) -> Result<Vec<DbRole>, sqlx::Error> {
        sqlx::query_as!(
            DbRole,
            "SELECT
                colour,
                deleted,
                flags,
                guild_id,
                hoist,
                icon as \"icon: Json<ImageHash>\",
                managed,
                mentionable,
                name,
                permissions,
                position,
                role_id,
                tags as \"tags: Json<RoleTags>\",
                unicode_emoji
            FROM guild_roles WHERE guild_id = $1",
            guild_id,
        )
        .fetch_all(&self.pool)
        .await
    }
}
