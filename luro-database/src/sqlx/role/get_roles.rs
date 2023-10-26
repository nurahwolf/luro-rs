use futures_util::TryStreamExt;
use sqlx::types::Json;
use twilight_model::{
    guild::{Permissions, RoleFlags, RoleTags},
    id::{marker::GuildMarker, Id},
    util::ImageHash,
};

use crate::{luro::luro_role_data::LuroRoleData, LuroDatabase, LuroRole};

impl LuroDatabase {
    pub async fn get_guild_roles(&self, guild_id: Id<GuildMarker>) -> anyhow::Result<Vec<LuroRole>> {
        let mut roles = vec![];

        while let Ok(Some(db_role)) = sqlx::query_file!("queries/guild_roles/get_roles.sql", guild_id.get() as i64)
            .fetch(&self.pool)
            .try_next()
            .await
        {
            roles.push(LuroRole {
                data: Some(LuroRoleData { deleted: db_role.deleted }),
                colour: db_role.colour as u32,
                hoist: db_role.hoist,
                icon: match db_role.icon {
                    Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                role_id: Id::new(db_role.role_id as u64),
                guild_id,
                managed: db_role.managed,
                mentionable: db_role.mentionable,
                name: db_role.role_name,
                permissions: Permissions::from_bits_retain(db_role.permissions as u64),
                position: db_role.position,
                flags: RoleFlags::from_bits_retain(db_role.role_flags as u64),
                tags: db_role.tags.map(|x| x.0),
                unicode_emoji: db_role.unicode_emoji,
            })
        }

        Ok(roles)
    }
}
