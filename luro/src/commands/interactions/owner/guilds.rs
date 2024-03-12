use std::fmt::Write;

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "guilds", desc = "Information about all guilds")]
pub struct Guilds {
    /// Optionally include the guild ID
    show_id: Option<bool>,
}

impl LuroCommand for Guilds {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut response = ctx.acknowledge_interaction(false).await?;
        let mut guild_string = String::new();
        for guild in ctx.database.guilds_fetch().await? {
            match self.show_id.unwrap_or_default() {
                true => writeln!(guild_string, "- {} - <#{1}> - {1}", guild.name, guild.guild_id)?,
                false => writeln!(guild_string, "- {} - <#{}>", guild.name, guild.guild_id)?,
            }
        }

        response.embed(|embed| {
            embed
                .title("All the guilds that I am in")
                .description(guild_string)
                .colour(ctx.accent_colour())
        });
        ctx.response_send(response).await
    }
}
