use crate::{LuroDatabase, LuroRole};
use sqlx::types::Json;
use twilight_model::guild::{Permissions, RoleFlags, RoleTags};
use twilight_model::id::marker::{GuildMarker, RoleMarker};
use twilight_model::id::Id;

impl LuroDatabase {
    pub async fn get_role(&self, guild_id: &Id<GuildMarker>, role_id: &Id<RoleMarker>) -> Result<Option<LuroRole>, sqlx::Error> {
        sqlx::query_file!("queries/guild_roles/get_role.sql", guild_id.get() as i64, role_id.get() as i64)
            .fetch_optional(&self.pool)
            .await
            .map(|x| match x {
                Some(db_role) => Some(LuroRole {
                    deleted: db_role.deleted,
                    guild_id: Id::new(db_role.guild_id as u64),
                    colour: db_role.colour as u32,
                    hoist: db_role.hoist,
                    icon: db_role.icon,
                    id: Id::new(db_role.role_id as u64),
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
