use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{
    marker::{GenericMarker, RoleMarker},
    Id
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
    guild: Option<Id<GenericMarker>>
}

impl LuroCommand for InfoRole {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let mut description = String::new();
        let mut embed = ctx.default_embed().await;
        let guild_id = ctx.interaction.guild_id.unwrap_or(match self.guild {
            Some(guild_id) => guild_id.cast(),
            None => {
                return ctx
                    .respond(|r| r.content("Could not find the guild the role is in").ephemeral())
                    .await
            }
        });
        let mut guild = ctx
            .framework
            .database
            .get_guild(&guild_id, &ctx.framework.twilight_client)
            .await?;

        guild.sort_roles();
        for luro_role in guild.role_positions.values() {
            if luro_role == &self.role {
                writeln!(description, "--> <@&{}> <--", luro_role)?;
                continue;
            }
            writeln!(description, "<@&{}>", luro_role)?;
        }
        embed.create_field("Sorted Roles", &description, true);
        embed
            .title(format!("{}'s roles", guild.name))
            .colour(ctx.accent_colour().await);

        ctx.respond(|r| r.add_embed(embed)).await
    }
}
