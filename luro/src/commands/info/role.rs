use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedMentionable};

use crate::models::RoleOrdering;

use crate::interaction::LuroSlash;
use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand)]
#[command(name = "role", desc = "Information about a role")]
pub struct InfoRole {
    /// The role to get
    role: ResolvedMentionable
}

impl LuroCommand for InfoRole {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut description: String = String::new();
        let role = match ctx.framework.twilight_cache.role(self.role.id().cast()) {
            Some(role) => role,
            None => return ctx.respond(|r| r.content("No role found! Sorry...").ephemeral()).await
        };

        {
            let roles = ctx.framework.twilight_client.roles(role.guild_id()).await?.model().await?;
            let mut roles: Vec<_> = roles.iter().map(RoleOrdering::from).collect();
            roles.sort_by(|a, b| b.cmp(a));
            for guild_role in roles {
                if guild_role.id == role.id {
                    writeln!(description, "--> <@&{}> <--", guild_role.id)?;
                    continue;
                }
                writeln!(description, "<@&{}>", guild_role.id)?;
            }
        }
        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|r| r.embed(|e| e.description(description).title(role.name.clone()).colour(accent_colour)))
            .await
    }
}
