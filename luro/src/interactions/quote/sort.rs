use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "sort", desc = "Sort the list of quotes (Owner Only)!")]
pub struct Sort {}

impl LuroCommand for Sort {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let mut quotes = ctx.framework.database.get_quotes().await?;

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

        ctx.framework.database.save_quotes(quotes).await?;

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|response| {
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
