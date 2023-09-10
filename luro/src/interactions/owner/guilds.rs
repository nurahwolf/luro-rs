use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;
use luro_model::database_driver::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "guilds", desc = "Information about all guilds")]
pub struct OwnerGuildsCommand {
    /// Optionally include the guild ID
    show_id: Option<bool>,
}

impl LuroCommand for OwnerGuildsCommand {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let mut guilds = String::new();
        for guild in ctx.framework.twilight_cache.iter().guilds() {
            if let Some(show_id) = self.show_id && show_id {
                writeln!(guilds, "{} - <#{1}> - {1}", guild.name(), guild.id())?
            } else {
                writeln!(guilds, "{} - <#{}>", guild.name(), guild.id())?
            }
        }

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|r| {
            r.embed(|embed| {
                embed
                    .title("All the guilds that I am in")
                    .description(guilds)
                    .colour(accent_colour)
            })
        })
        .await
    }
}
