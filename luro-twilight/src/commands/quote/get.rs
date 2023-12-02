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
        let quote = match self.id {
            Some(quote_id) => ctx.database.quote_fetch(quote_id).await?,
            None => {
                let (sfw_quotes, nsfw_quotes) = ctx.database.driver.quotes_fetch().await?;
                tracing::info!(
                    "Total quotes returned: {} and {} (SFW and NSFW)",
                    sfw_quotes.len(),
                    nsfw_quotes.len()
                );
                match ctx.channel.nsfw.unwrap_or_default() {
                    true => nsfw_quotes.choose(&mut rand::thread_rng()).cloned(),
                    false => sfw_quotes.choose(&mut rand::thread_rng()).cloned(),
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

        let quote_author = ctx.fetch_user(quote.message.author.user_id).await?;

        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .colour(ctx.accent_colour())
                    .description(quote.message.content)
                    .author(|author| {
                        author
                            .name(format!("{} - Quote {}", quote_author.name(), quote.quote_id))
                            .icon_url(quote_author.avatar_url());
                        match quote.message.guild_id {
                            Some(guild_id) => author.url(format!(
                                "https://discord.com/channels/{guild_id}/{}/{}",
                                quote.channel_id, quote.quote_id
                            )),
                            None => author.url(format!("https://discord.com/channels/{}/{}", quote.channel_id, quote.quote_id)),
                        }
                    })
            })
        })
        .await
    }
}
