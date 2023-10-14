use twilight_model::guild::RoleTags;

use sqlx::types::Json;
use twilight_model::util::ImageHash;

use crate::{DbRole, LuroDatabase};

impl LuroDatabase {
    pub async fn delete_role(&self, role_id: i64) -> Result<DbRole, sqlx::Error> {
        sqlx::query_as!(
            DbRole,
            "INSERT INTO guild_roles (
                deleted,
                role_id
            ) VALUES
                ($1, $2)
            ON CONFLICT
                (role_id)
            DO UPDATE SET
                deleted = $1
            RETURNING
                colour,
                deleted,
                hoist,
                icon as \"icon: Json<ImageHash>\",
                role_id,
                guild_id,
                managed,
                mentionable,
                name,
                permissions,
                position,
                flags,
                tags as \"tags: Json<RoleTags>\",
                unicode_emoji",
            true,
            role_id,
        )
        .fetch_one(&self.pool)
        .await
    }
}
