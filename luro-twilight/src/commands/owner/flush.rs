use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "flush",
    desc = "Drop the database cache and reinitialise it, useful for if data has changed on the backend"
)]
pub struct Flush {}

#[async_trait]
impl LuroCommandTrait for Flush {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
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
