use luro_model::luro_message::LuroMessage;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::marker::ChannelMarker;
use twilight_model::id::Id;
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::functions::client_fetch;
use crate::interaction::LuroSlash;
use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "get_message", desc = "Gets a particular message from the cache, or user's data")]
pub struct OwnerGetMessage {
    /// The message ID to get
    message_id: String,
    /// If defined, attempts to find the message from this user's data if not found in the cache
    user: Option<ResolvedUser>,
    /// If defined, attempts to use the client to fetch the message
    channel_id: Option<Id<ChannelMarker>>,
    /// Convert the message to a [LuroMessage].
    convert: Option<bool>
}

impl LuroCommand for OwnerGetMessage {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let accent_colour = ctx.accent_colour().await;
        let message_id = Id::new(self.message_id.parse()?);
        let mut embed = EmbedBuilder::new().color(accent_colour);

        let (slash_author, channel_id, message_id, message) = if let Some(channel_id) = self.channel_id {
            let message = ctx
                .framework
                .twilight_client
                .message(channel_id, message_id)
                .await?
                .model()
                .await?;

            (
                client_fetch(&ctx.framework, message.guild_id, message.author.id).await?,
                message.channel_id,
                message_id,
                Some(message)
            )
        } else {
            match ctx.framework.twilight_cache.message(message_id) {
                Some(message) => {
                    embed = embed.description(message.content());
                    (
                        client_fetch(&ctx.framework, message.guild_id(), message.author()).await?,
                        message.channel_id(),
                        message.id(),
                        None
                    )
                }
                None => {
                    let user = match self.user {
                        Some(user) => user,
                        None => {
                            return ctx
                                .respond(|r| {
                                    r.content("Message not found! Try specifying a user ID if you know who sent it.")
                                        .ephemeral()
                                })
                                .await
                        }
                    };
                    let user_data = ctx.framework.database.get_user(&user.resolved.id).await?;
                    let message = match user_data.messages.get(&message_id) {
                        Some(message) => message,
                        None => {
                            return ctx
                                .respond(|r| {
                                    r.content("Looks like the user does not have the message ID you provided, sorry.")
                                        .ephemeral()
                                })
                                .await
                        }
                    };

                    if let Some(content) = &message.content {
                        embed = embed.description(content)
                    }

                    (
                        client_fetch(&ctx.framework, ctx.interaction.guild_id, user.resolved.id).await?,
                        message.channel_id,
                        message.id,
                        None
                    )
                }
            }
        };

        embed = embed.author(EmbedAuthorBuilder::new(slash_author.name).icon_url(ImageSource::url(slash_author.avatar)?));
        embed = embed.field(EmbedFieldBuilder::new("Channel", format!("<#{}>", channel_id)).inline());
        embed = embed.field(EmbedFieldBuilder::new("Message ID", message_id.to_string()).inline());

        match self.convert.unwrap_or_default() {
            true => {
                let message = match message {
                    Some(message) => message,
                    None => {
                        ctx.framework
                            .twilight_client
                            .message(channel_id, message_id)
                            .await?
                            .model()
                            .await?
                    }
                };

                let mut luro_message = LuroMessage::from(message.clone());
                if let Some(member) = message.member {
                    luro_message.add_partialmember(&message.author, &member);
                }
                if let Some(guild_id) = message.guild_id && let Ok(member) = ctx.framework.twilight_client.guild_member(guild_id, message.author.id).await {
                    luro_message.add_member(&message.author, &member.model().await?, &guild_id);
                }
                let toml = toml::to_string_pretty(&luro_message)?;
                ctx.respond(|r| {
                    r.embed(|embed| {
                        embed
                            .colour(accent_colour)
                            .title("LuroMessage")
                            .description(format!("```toml\n{toml}\n```"))
                    })
                    .add_embed(embed.build())
                    .ephemeral()
                })
                .await
            }
            false => ctx.respond(|r| r.add_embed(embed.build()).ephemeral()).await
        }
    }
}
