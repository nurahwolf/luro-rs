use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "sort", desc = "Sort the list of quotes (Owner Only)!")]
pub struct Sort {}

#[async_trait]
impl LuroCommandTrait for Sort {
    async fn handle_interaction(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut quotes = ctx.database.get_quotes().await?;

        // Remove duplicates and new IDs
        let mut iteration = 0;
        let mut new_quotes = vec![];
        for quote in quotes.values() {
            if !new_quotes.contains(quote) {
                new_quotes.insert(iteration, quote.clone());
                iteration += 1;
            };
        }

        iteration = 0;
        quotes.clear();
        for quote in new_quotes {
            quotes.insert(iteration, quote);
            iteration += 1;
        }

        ctx.database.save_quotes(quotes).await?;

        let accent_colour = interaction.accent_colour(&ctx).await;
        interaction
            .respond(&ctx, |response| {
                response.embed(|embed| {
                    embed
                        .colour(accent_colour)
                        .title("Quote Sorted")
                        .description(format!("There are now a total of {iteration} quotes!"))
                })
            })
            .await
    }
}
