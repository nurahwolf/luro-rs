use crate::{DbRole, LuroDatabase};
use sqlx::types::Json;
use twilight_model::guild::RoleTags;
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::Id;

impl LuroDatabase {
    pub async fn get_role(&self, role_id: &Id<RoleMarker>) -> Result<Option<DbRole>, sqlx::Error> {
        sqlx::query_as!(
            DbRole,
            "SELECT
                colour,
                deleted,
                flags,
                guild_id,
                hoist,
                icon,
                managed,
                mentionable,
                name,
                permissions,
                position,
                role_id,
                tags as \"tags: Json<RoleTags>\",
                unicode_emoji
            FROM guild_roles WHERE role_id = $1",
            role_id.get() as i64,
        )
        .fetch_optional(&self.pool)
        .await
    }
}
