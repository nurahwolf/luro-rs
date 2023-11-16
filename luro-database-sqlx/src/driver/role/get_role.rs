use luro_model::types::{Role, RoleData};
use sqlx::types::Json;
use twilight_model::guild::{Permissions, RoleFlags, RoleTags};
use twilight_model::id::marker::{GuildMarker, RoleMarker};
use twilight_model::id::Id;
use twilight_model::util::ImageHash;

impl crate::SQLxDriver {
    pub async fn role_fetch(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> anyhow::Result<Option<Role>> {
        let query = sqlx::query_file!("queries/guild/guild_fetch_role.sql", guild_id.get() as i64, role_id.get() as i64)
            .fetch_optional(&self.pool)
            .await?;

        Ok(match query {
            Some(db_role) => Some(Role {
                data: Some(RoleData { deleted: db_role.deleted }),
                colour: db_role.colour as u32,
                hoist: db_role.hoist,
                icon: match db_role.icon {
                    Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                role_id: Id::new(db_role.role_id as u64),
                guild_id: Id::new(db_role.guild_id as u64),
                managed: db_role.managed,
                mentionable: db_role.mentionable,
                name: db_role.role_name,
                permissions: Permissions::from_bits_retain(db_role.permissions as u64),
                position: db_role.position,
                flags: RoleFlags::from_bits_retain(db_role.role_flags as u64),
                tags: db_role.tags.map(|x| x.0),
                unicode_emoji: db_role.unicode_emoji,
            }),
            None => None,
        })
    }
}
