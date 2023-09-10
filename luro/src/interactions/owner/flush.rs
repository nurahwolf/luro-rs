use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "flush",
    desc = "Drop the database cache and reinitialise it, useful for if data has changed on the backend"
)]
pub struct Flush {}

impl LuroCommand for Flush {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let errors = ctx.framework.database.flush().await?;
        let accent_colour = ctx.accent_colour().await;

        ctx.respond(|r| {
            r.embed(|embed| {
                match errors.is_empty() {
                    true => embed.description("Flushed all data with no errors!"),
                    false => embed.description(format!("Flushed with the following errors:\n```\n{errors}\n```")),
                };
                embed.title("Database flushed!").colour(accent_colour)
            })
            .ephemeral()
        })
        .await
    }
}
