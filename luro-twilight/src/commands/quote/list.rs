use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::{database::drivers::LuroDatabaseDriver, message::LuroMessageSource};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "list", desc = "List some quotes!")]
pub struct List {}

#[async_trait]
impl LuroCommandTrait for List {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let quotes = ctx.database.get_quotes().await?;
        let mut quotes_string = String::new();

        for (id, quote) in quotes.into_iter() {
            let content = quote.content;
            if let Some(content) = content.lines().next() {
                let mut content = content.to_string();
                content = match quote.source {
                    LuroMessageSource::Custom => format!("\n- `{id} (Custom)` - {content}"),
                    _ => format!("\n- `{id}` - {content}"),
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
                            false => split += 1,
                        }
                    }
                    content.push_str("...");
                }
                quotes_string.push_str(&content);
            }
        }

        let accent_colour = interaction.accent_colour(&ctx).await;
        interaction
            .respond(&ctx, |response| {
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
