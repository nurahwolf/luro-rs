use twilight_model::guild::RoleTags;

use sqlx::types::Json;
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::Id;
use twilight_model::util::ImageHash;

use crate::{DatabaseRole, LuroDatabase};

impl LuroDatabase {
    pub async fn delete_role(&self, role_id: Id<RoleMarker>) -> Result<DatabaseRole, sqlx::Error> {
        sqlx::query_as!(
            DatabaseRole,
            "INSERT INTO roles (
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
                managed,
                mentionable,
                name,
                permissions,
                position,
                flags,
                tags as \"tags: Json<RoleTags>\",
                unicode_emoji",
            true,
            role_id.get() as i64,
        )
        .fetch_one(&self.0)
        .await
    }
}
