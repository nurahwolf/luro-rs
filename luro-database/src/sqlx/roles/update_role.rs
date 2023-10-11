use twilight_model::guild::RoleTags;

use sqlx::types::Json;
use twilight_model::util::ImageHash;

use crate::{DatabaseRole, LuroDatabase};

impl LuroDatabase {
    pub async fn update_role(&self, role: impl Into<DatabaseRole>) -> Result<DatabaseRole, sqlx::Error> {
        let role: DatabaseRole = role.into();

        let query = sqlx::query_as!(
            DatabaseRole,
            "INSERT INTO roles (
                colour,
                hoist,
                icon,
                role_id,
                managed,
                mentionable,
                name,
                permissions,
                position,
                flags,
                tags,
                unicode_emoji
            ) VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT
                (role_id)
            DO UPDATE SET
                colour = $1,
                hoist = $2,
                icon = $3,
                managed = $5,
                mentionable = $6,
                name = $7,
                permissions = $8,
                position = $9,
                flags = $10,
                tags = $11,
                unicode_emoji = $12
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
            role.colour,
            role.hoist,
            role.icon as _,
            role.role_id,
            role.managed,
            role.mentionable,
            role.name,
            role.permissions,
            role.position,
            role.flags,
            role.tags as _,
            role.unicode_emoji,
        );

        query.fetch_one(&self.0).await
    }
}
