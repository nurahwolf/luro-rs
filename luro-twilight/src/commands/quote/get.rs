use luro_framework::Luro;
use rand::seq::SliceRandom;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "get", desc = "Get a memorable quote!")]
pub struct Get {
    /// The quote to get! Gets random if not set
    id: Option<i64>,
    // /// Set this to send a webhook with the message
    // puppet: Option<bool>,
}

impl luro_framework::LuroCommand for Get {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let (quote, quote_id) = match self.id {
            Some(quote_id) => (ctx.database.quote_fetch(quote_id).await?, quote_id),
            None => {
                let (sfw_quotes, nsfw_quotes) = ctx.database.driver.quotes_fetch().await?;
                let chosen_quote = match ctx.channel.nsfw.unwrap_or_default() {
                    true => nsfw_quotes.choose(&mut rand::thread_rng()),
                    false => sfw_quotes.choose(&mut rand::thread_rng()),
                };

                match chosen_quote {
                    Some(raw_quote) => (ctx.database.quote_fetch(raw_quote.quote_id).await?, raw_quote.quote_id),
                    None => (None, 0),
                }
            }
        };

        let quote = match quote {
            Some(quote) => quote,
            None => {
                return ctx
                    .respond(|r| r.content("Sorry! The quote was not found in my database :(").ephemeral())
                    .await
            }
        };

        ctx.respond(|response| {
            response.embed(|embed| {
                embed.colour(ctx.accent_colour()).description(quote.content).author(|author| {
                    author
                        .name(format!("{} - Quote {quote_id}", quote.author.name()))
                        .icon_url(quote.author.avatar_url());
                    match quote.guild_id {
                        Some(guild_id) => {
                            author.url(format!("https://discord.com/channels/{guild_id}/{}/{}", quote.channel_id, quote.id))
                        }
                        None => author.url(format!("https://discord.com/channels/{}/{}", quote.channel_id, quote.id)),
                    }
                })
            })
        })
        .await
    }
}
