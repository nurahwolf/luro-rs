use luro_model::{database::drivers::LuroDatabaseDriver, message::LuroMessageSource};
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "list", desc = "List some quotes!")]
pub struct List {}

impl LuroCommand for List {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let quotes = ctx.framework.database.get_quotes().await?;
        let mut quotes_string = String::new();

        for (id, quote) in quotes.into_iter() {
            let content = quote.content;
            if let Some(content) = content.lines().next() {
                let mut content = content.to_string();
                content = match quote.source {
                    LuroMessageSource::Custom => format!("\n- `{id} (Custom)` - {content}"),
                    _ => format!("\n- `{id}` - {content}")
                };
                if content.len() > 75 {
                    let mut split = 75;
                    let mut success = false;
                    while !success {
                        match content.is_char_boundary(split) {
                            true => {
                                content.truncate(split);
                                success = true
                            }
                            false => split += 1
                        }
                    }
                    content.push_str("...");
                }
                quotes_string.push_str(&content);
            }
        }

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .colour(accent_colour)
                    .title("Some quotes to choose from...")
                    .description(quotes_string)
            })
        })
        .await
    }
}
