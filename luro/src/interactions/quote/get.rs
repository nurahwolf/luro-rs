use anyhow::Context;

use luro_model::database::drivers::LuroDatabaseDriver;
use rand::Rng;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand, models::LuroWebhook};

#[derive(CommandModel, CreateCommand)]
#[command(name = "get", desc = "Get a memorable quote!")]
pub struct Get {
    /// The quote to get! Gets random if not set
    id: Option<i64>,
    /// Set this to send a webhook with the message
    puppet: Option<bool>
}

impl LuroCommand for Get {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let id = match self.id {
            Some(id) => id.try_into().context("Expected to convert i64 to usize")?,
            None => {
                let total = ctx.framework.database.get_quotes().await?.len();
                rand::thread_rng().gen_range(0..total)
            }
        };

        let quote = match ctx.framework.database.get_quote(id).await {
            Ok(quote) => quote,
            Err(_) => return ctx.respond(|r| r.content("Sorry! Quote was not found :(").ephemeral()).await
        };
        let user = ctx.framework.database.get_user(&quote.author).await?;

        if self.puppet.unwrap_or_default() {
            let luro_webhook = LuroWebhook::new(ctx.framework.clone());
            let webhook = luro_webhook
                .get_webhook(
                    ctx.interaction
                        .channel
                        .clone()
                        .context("Could not get channel the interaction is in")?
                        .id
                )
                .await?;

            let webhook_token = match webhook.token {
                Some(token) => token,
                None => match ctx.framework.twilight_client.webhook(webhook.id).await?.model().await?.token {
                    Some(token) => token,
                    None => {
                        return ctx
                            .respond(|r| {
                                r.content("Sorry, I can't setup a webhook here. Probably missing perms.")
                                    .ephemeral()
                            })
                            .await
                    }
                }
            };

            ctx.framework
                .twilight_client
                .execute_webhook(webhook.id, &webhook_token)
                .username(&user.name)
                .avatar_url(&user.avatar())
                .content(&quote.content)
                .await?;

            return ctx.respond(|response| response.content("Puppetted!").ephemeral()).await;
        }

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|response| {
            response.embed(|embed| {
                embed.colour(accent_colour).description(quote.content).author(|author| {
                    author.name(format!("{} - Quote {id}", user.name())).icon_url(user.avatar());
                    match quote.guild_id {
                        Some(guild_id) => author.url(format!(
                            "https://discord.com/channels/{guild_id}/{}/{}",
                            quote.channel_id, quote.id
                        )),
                        None => author.url(format!("https://discord.com/channels/{}/{}", quote.channel_id, quote.id))
                    }
                })
            })
        })
        .await
    }
}
