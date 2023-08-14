use anyhow::Context;
use luro_model::{luro_message::LuroMessage, slash_user::SlashUser};
use rand::Rng;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};

use crate::{functions::client_fetch, interaction::LuroSlash, luro_command::LuroCommand, models::LuroWebhook};

#[derive(CommandModel, CreateCommand)]
#[command(name = "quote", desc = "Get or save some quotes")]
pub enum QuoteCommands {
    #[command(name = "get")]
    GetQuote(GetQuote),
    #[command(name = "add")]
    SaveQuote(SaveQuote)
}

impl LuroCommand for QuoteCommands {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        match self {
            Self::GetQuote(command) => command.run_command(ctx).await,
            Self::SaveQuote(command) => command.run_command(ctx).await
        }
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "get", desc = "Get a memorable quote!")]
pub struct GetQuote {
    /// The quote to get! Gets random if not set
    id: Option<i64>,
    /// Set this to send a webhook with the message
    puppet: Option<bool>
}

impl LuroCommand for GetQuote {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
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

        let slash_user = match quote.luro_user {
            Some(slash_user) => slash_user,
            None => match quote.author {
                Some(user) => SlashUser::from(user),
                None => match quote.author_id {
                    Some(author_id) => client_fetch(&ctx.framework, ctx.interaction.guild_id, author_id).await?,
                    None => Default::default()
                }
            }
        };

        if self.puppet.unwrap_or_default() {
            let luro_webhook = LuroWebhook::new(&ctx.framework).await?;
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
                .username(&slash_user.name)
                .avatar_url(&slash_user.avatar)
                .content(&quote.content.unwrap_or_default())
                .await?;

            return ctx.respond(|response| response.content("Puppetted!").ephemeral()).await;
        }

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .colour(accent_colour)
                    .description(quote.content.unwrap_or_default())
                    .author(|author| {
                        author.name(slash_user.name).icon_url(slash_user.avatar);
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

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Save what someone said!")]
pub struct SaveQuote {
    /// The message ID to save
    id: String,
    /// Only set this if Luro can't find the message in the cache.
    channel: Option<Id<ChannelMarker>>
}

impl LuroCommand for SaveQuote {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction = &ctx.interaction;
        let channel_id = self.channel.unwrap_or(interaction.channel.as_ref().unwrap().id);
        let accent_colour = ctx.accent_colour().await;
        let id = Id::new(self.id.parse()?);
        let quoted_user;

        let mut quote = match ctx.framework.twilight_client.message(channel_id, id).await {
            Ok(message) => {
                let message = message.model().await?;
                quoted_user = message.author.clone();
                LuroMessage::from(message)
            }
            Err(_) => {
                match ctx.framework.twilight_cache.message(id) {
                    Some(message) => {
                        let mut quote = LuroMessage::from(message.clone());
                        // Add some more stuff that is not in the cache
                        if let Some(user) = ctx.framework.twilight_cache.user(message.author()) {
                            quoted_user = user.clone();
                            quote.add_user(&user.clone());
                        } else {
                            let user = ctx.framework.twilight_client.user(message.author()).await?.model().await?;
                            quoted_user = user.clone();
                            quote.add_user(&user.clone());
                        }
                        quote
                    },
                    None => return ctx
                    .respond(|r| {
                        r.content("Sorry! Could not find that message. You sure you gave me the right ID?\nTry specifying the exact channel with the optional parameter if I'm struggling. If I still can't, it's probably because I don't have access to the channel.")
                            .ephemeral()
                    })
                    .await,
                }
            }
        };

        if let Some(guild_id) = interaction.guild_id {
            if let Ok(member) = ctx.framework.twilight_client.guild_member(guild_id, quoted_user.id).await {
                quote.add_member(&quoted_user, &member.model().await?);
            }
        }

        // SAFETY: We know we are safe to unwrap here as none of the matches above will result in this field not being set
        let slash_user = SlashUser::from(quoted_user);

        let id = ctx.framework.database.get_quotes().await?.len();
        ctx.framework.database.save_quote(id + 1, &quote).await?;

        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .colour(accent_colour)
                    .description(quote.content.unwrap_or_default())
                    .author(|author| {
                        author.name(slash_user.name).icon_url(slash_user.avatar);
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
