use anyhow::Context;

use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use rand::Rng;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "get", desc = "Get a memorable quote!")]
pub struct Get {
    /// The quote to get! Gets random if not set
    id: Option<i64>,
    /// Set this to send a webhook with the message
    puppet: Option<bool>
}
#[async_trait]

impl LuroCommandTrait for Get {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;

        let id = match data.id {
            Some(id) => id.try_into().context("Expected to convert i64 to usize")?,
            None => {
                let total = ctx.database.get_quotes().await?.len();
                rand::thread_rng().gen_range(0..total)
            }
        };

        let quote = match ctx.database.get_quote(id).await {
            Ok(quote) => quote,
            Err(_) => {
                return interaction
                    .respond(&ctx, |r| r.content("Sorry! Quote was not found :(").ephemeral())
                    .await
            }
        };
        let user = ctx.database.get_user(&quote.author, false).await?;

        if data.puppet.unwrap_or_default() {
            let webhook = ctx
                .get_webhook(
                    interaction
                        .channel
                        .clone()
                        .context("Could not get channel the interaction is in")?
                        .id
                )
                .await?;

            let webhook_token = match webhook.token {
                Some(token) => token,
                None => match ctx.twilight_client.webhook(webhook.id).await?.model().await?.token {
                    Some(token) => token,
                    None => {
                        return interaction
                            .respond(&ctx, |r| {
                                r.content("Sorry, I can't setup a webhook here. Probably missing perms.")
                                    .ephemeral()
                            })
                            .await
                    }
                }
            };

            ctx.twilight_client
                .execute_webhook(webhook.id, &webhook_token)
                .username(&user.name)
                .avatar_url(&user.avatar())
                .content(&quote.content)
                .await?;

            return interaction
                .respond(&ctx, |response| response.content("Puppetted!").ephemeral())
                .await;
        }

        let accent_colour = interaction.accent_colour(&ctx).await;
        interaction
            .respond(&ctx, |response| {
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
