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
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let mut guilds = String::new();
        for guild in ctx.cache.iter().guilds() {
            if self.show_id.unwrap_or_default() {
                writeln!(guilds, "{} - <#{1}> - {1}", guild.name(), guild.id())?
            } else {
                writeln!(guilds, "{} - <#{}>", guild.name(), guild.id())?
            }
        }

        let accent_colour = ctx.accent_colour();
        ctx.respond(|r| r.embed(|embed| embed.title("All the guilds that I am in").description(guilds).colour(accent_colour)))
            .await
    }
}
