use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::{database_driver::LuroDatabaseDriver, message::LuroMessage, user::LuroUser};
use tracing::debug;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Save what someone said!")]
pub struct Add {
    /// The message ID to save
    id: String,
    /// Only set this if Luro can't find the message in the cache.
    channel: Option<Id<ChannelMarker>>,
}

#[async_trait]
impl LuroCommandTrait for Add {
    async fn handle_interaction(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;

        let channel_id = data.channel.unwrap_or(interaction.channel.id);
        let accent_colour = interaction.accent_colour(&ctx).await;
        let id = Id::new(data.id.parse()?);
        let quoted_user;

        let quote = match ctx.twilight_client.message(channel_id, id).await {
            Ok(message) => {
                let message = message.model().await?;
                quoted_user = message.author.clone();
                LuroMessage::from(message)
            }
            Err(_) => {
                debug!("Failed to get message via twilight, so lookign in the cache");
                match ctx.cache.message(id) {
                    Some(message) => {
                        let quote = LuroMessage::from(message.clone());
                        // Add some more stuff that is not in the cache
                        if let Some(user) = ctx.cache.user(message.author()) {
                            quoted_user = user.clone();
                        } else {
                            let user = ctx.twilight_client.user(message.author()).await?.model().await?;
                            quoted_user = user.clone();
                        }
                        quote
                    },
                    None => return interaction
                    .respond(&ctx, |r| {
                        r.content("Sorry! Could not find that message. You sure you gave me the right ID?\nTry specifying the exact channel with the optional parameter if I'm struggling. If I still can't, it's probably because I don't have access to the channel.")
                            .ephemeral()
                    })
                    .await,
                }
            }
        };

        let slash_user = LuroUser::from(&quoted_user);

        let local_quotes = ctx.database.get_quotes().await?;
        let local_quote_id = local_quotes.len();

        ctx.database.save_quote(local_quote_id, quote.clone()).await?;

        interaction
            .respond(&ctx, |response| {
                response.embed(|embed| {
                    embed.colour(accent_colour).description(quote.content).author(|author| {
                        author
                            .name(format!("{} - Quote {local_quote_id}", slash_user.name))
                            .icon_url(slash_user.avatar());
                        match quote.guild_id {
                            Some(guild_id) => author.url(format!(
                                "https://discord.com/channels/{guild_id}/{}/{}",
                                quote.channel_id, quote.id
                            )),
                            None => author.url(format!("https://discord.com/channels/{}/{}", quote.channel_id, quote.id)),
                        }
                    })
                })
            })
            .await
    }
}
