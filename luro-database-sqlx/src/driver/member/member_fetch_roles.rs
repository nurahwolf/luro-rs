use std::collections::HashMap;

use futures_util::TryStreamExt;
use luro_model::types::{Role, RoleData};
use sqlx::types::Json;
use twilight_model::guild::{Permissions, RoleFlags, RoleTags};
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};
use twilight_model::util::ImageHash;

impl crate::SQLxDriver {
    pub async fn member_fetch_roles(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> anyhow::Result<HashMap<Id<RoleMarker>, luro_model::types::Role>> {
        let mut member_roles = HashMap::new();

        let mut query =
            sqlx::query_file!("queries/member/member_fetch_roles.sql", guild_id.get() as i64, user_id.get() as i64,).fetch(&self.pool);

        while let Ok(Some(role)) = query.try_next().await {
            member_roles.insert(
                Id::new(role.role_id as u64),
                Role {
                    data: Some(RoleData { deleted: role.deleted }),
                    colour: role.colour as u32,
                    hoist: role.hoist,
                    icon: match role.icon {
                        Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                        None => None,
                    },
                    role_id: Id::new(role.role_id as u64),
                    guild_id: Id::new(role.guild_id as u64),
                    managed: role.managed,
                    mentionable: role.mentionable,
                    name: role.role_name,
                    permissions: Permissions::from_bits_retain(role.permissions as u64),
                    position: role.position,
                    flags: RoleFlags::from_bits_retain(role.role_flags as u64),
                    tags: role.tags.map(|x| x.0),
                    unicode_emoji: role.unicode_emoji,
                },
            );
        }

        Ok(member_roles)
    }
}
