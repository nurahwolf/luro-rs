use futures_util::StreamExt;
use sqlx::types::Json;
use twilight_model::guild::RoleTags;

use crate::{database::sqlx::Database, user::MemberContext};

impl Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn fetch_member_roles<'a>(&'a self, member: &'a mut MemberContext) -> bool {
        let g_id = member.guild_id.get() as i64;
        let u_id = member.user_id().get() as i64;
        let mut roles_found = false;
        let mut roles_query = sqlx::query_file!("queries/member/member_fetch_roles.sql", g_id, u_id).fetch(&self.pool);

        while let Some(role) = roles_query.next().await {
            let role = match role {
                Ok(role) => role,
                Err(why) => {
                    tracing::error!(?why, "Database failed to get a role");
                    continue;
                }
            };

            // Create the role
            let role = twilight_model::guild::Role {
                color: role.colour as u32,
                hoist: role.hoist,
                icon: role
                    .icon
                    .map(|icon| {
                        twilight_model::util::ImageHash::parse(icon.as_bytes()).map_or_else(
                            |why| {
                                tracing::warn!("role_icon - Failed to parse image: {why}");
                                None
                            },
                            |img| Some(img),
                        )
                    })
                    .flatten(),
                id: twilight_model::id::Id::new(role.role_id as u64),
                managed: role.managed,
                mentionable: role.mentionable,
                name: role.role_name,
                permissions: twilight_model::guild::Permissions::from_bits_retain(role.permissions as u64),
                position: role.position,
                flags: twilight_model::guild::RoleFlags::from_bits_retain(role.role_flags as u64),
                tags: role.tags.map(|x| x.0),
                unicode_emoji: role.unicode_emoji,
            };

            // Handle if it is an everyone role
            match role.id.get() == member.guild_id.get() {
                true => {
                    member.twilight_member.roles.push(role.id);
                    member.everyone_role = Some(role)
                }
                false => {
                    member.twilight_member.roles.push(role.id);
                    member.roles.push(role)
                }
            }

            roles_found = true;
        }

        member
            .roles
            .sort_by(|a, b| a.position.cmp(&b.position).then(a.id.get().cmp(&b.id.get())).reverse());

        roles_found
    }
}
