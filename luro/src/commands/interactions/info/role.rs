use std::fmt::Write;

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::response::SimpleResponse;
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
    /// Show the role position
    show_position: bool,
    /// The guild to get the role from
    guild: Option<Id<GenericMarker>>,
}

impl LuroCommand for InfoRole {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut embed = ctx.default_embed().await;
        let guild = match self.guild {
            Some(guild_requested) => ctx.get_guild(guild_requested.cast()).await?,
            None => match &ctx.guild {
                Some(guild) => guild.clone(),
                None => return ctx.simple_response(SimpleResponse::NotGuild).await,
            },
        };
        let guild_roles = ctx.get_guild_roles(guild.guild_id).await?;
        let role = ctx.database.role_fetch(guild.guild_id, self.role).await?;

        embed.title(format!("{}'s roles", guild.name));
        embed.create_field("Role Position", &format!("`{}`", role.position), true);

        for role in &guild_roles {
            if role.role_id == self.role {
                embed.create_field("Role Name", &format!("`{}`", role.name), true);
                embed.create_field("Role Colour", &format!("`{}`", role.colour), true);
            }
        }

        if self.show_position {
            let mut description = String::new();
            for role in guild_roles {
                if role.role_id == self.role {
                    writeln!(description, "--> <@&{}> <--", role.role_id)?;
                    continue;
                }
                writeln!(description, "<@&{}>", role.role_id)?;
            }
            embed.description(description);
        }

        ctx.respond(|r| r.add_embed(embed)).await
    }
}
