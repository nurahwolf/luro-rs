use std::fmt::Write;

use async_trait::async_trait;
use luro_framework::{command::ExecuteLuroCommand, CommandInteraction, interactions::InteractionTrait, Luro};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{
    marker::{GenericMarker, RoleMarker},
    Id,
};


#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "role", desc = "Information about a role")]
pub struct InfoRole {
    /// The role to get
    role: Id<RoleMarker>,
    /// The guild to get the role from
    guild: Option<Id<GenericMarker>>,
    /// Show guild roles as well
    guid_roles: Option<bool>,
}

#[async_trait]
impl ExecuteLuroCommand for InfoRole {
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => match self.guild {
                Some(guild_id) => guild_id.cast(),
                None => {
                    return ctx
                        .respond(|r| r.content("Could not find the guild the role is in").ephemeral())
                        .await
                }
            },
        };
        let mut guild = ctx.get_guild(&guild_id).await?;
        embed.title(format!("{}'s roles", guild.name));

        for (position, role_id) in &guild.role_positions {
            if role_id == &self.role {
                embed.create_field("Role Position", &format!("`{position}`"), true);
            }
        }

        if let Some(role) = guild.roles.get(&self.role) {
            embed.create_field("Role Name", &format!("`{}`", role.name), true);
            embed.create_field("Role Colour", &format!("`{}`", role.colour), true);
        }

        if self.guid_roles.unwrap_or_default() {
            let mut description = String::new();
            guild.sort_roles();
            for luro_role in guild.role_positions.values() {
                if luro_role == &self.role {
                    writeln!(description, "--> <@&{}> <--", luro_role)?;
                    continue;
                }
                writeln!(description, "<@&{}>", luro_role)?;
            }
            embed.description(description);
        }

        ctx.respond(|r| r.add_embed(embed)).await
    }
}
