use async_trait::async_trait;
use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedMentionable};

use crate::models::{LuroResponse, RoleOrdering};
use crate::LuroContext;

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand)]
#[command(name = "role", desc = "Information about a role")]
pub struct InfoRole {
    /// The role to get
    role: ResolvedMentionable
}

#[async_trait]
impl LuroCommand for InfoRole {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let mut embed;
        {
            let role = match ctx.twilight_cache.role(self.role.id().cast()) {
                Some(role) => role,
                None => {
                    slash.content("Role not found!").ephemeral();
                    return ctx.respond(&mut slash).await;
                }
            };
            embed = ctx.default_embed(&slash.interaction.guild_id);
            let mut description: String = String::new();

            embed = embed.title(&role.name);
            let roles = ctx.twilight_client.roles(role.guild_id()).await?.model().await?;
            let mut roles: Vec<_> = roles.iter().map(RoleOrdering::from).collect();
            roles.sort_by(|a, b| b.cmp(a));
            for guild_role in roles {
                if guild_role.id == role.id {
                    writeln!(description, "--> <@&{}> <--", guild_role.id)?;
                    continue;
                }
                writeln!(description, "<@&{}>", guild_role.id)?;
            }

            embed = embed.description(description)
        }

        slash.embed(embed.build())?;
        ctx.respond(&mut slash).await
    }
}
