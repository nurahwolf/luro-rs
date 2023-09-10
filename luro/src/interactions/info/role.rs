use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{
    marker::{GenericMarker, RoleMarker},
    Id,
};

use crate::interaction::LuroSlash;
use crate::luro_command::LuroCommand;
use luro_model::database::drivers::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand)]
#[command(name = "role", desc = "Information about a role")]
pub struct InfoRole {
    /// The role to get
    role: Id<RoleMarker>,
    /// The guild to get the role from
    guild: Option<Id<GenericMarker>>,
    /// Show guild roles as well
    guid_roles: Option<bool>,
}

impl LuroCommand for InfoRole {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        let guild_id = match ctx.interaction.guild_id {
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
        let mut guild = ctx.framework.database.get_guild(&guild_id).await?;
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
