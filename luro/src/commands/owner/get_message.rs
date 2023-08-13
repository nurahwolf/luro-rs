use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::Id;
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::models::SlashUser;

use crate::interaction::LuroSlash;
use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "get_message", desc = "Gets a particular message from the cache, or user's data")]
pub struct OwnerGetMessage {
    /// The message ID to get
    message_id: String,
    /// If defined, attempts to find the message from this user's data if not found in the cache
    user: Option<ResolvedUser>
}

impl LuroCommand for OwnerGetMessage {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let message_id = Id::new(self.message_id.parse()?);
        let mut embed = EmbedBuilder::new().color(ctx.accent_colour().await);
        let ((_, slash_author), channel_id, message_id) = match ctx.framework.twilight_cache.message(message_id) {
            Some(message) => {
                embed = embed.description(message.content());
                (
                    SlashUser::client_fetch_user(&ctx.framework, message.author()).await?,
                    message.channel_id(),
                    message.id()
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
                    SlashUser::client_fetch_user(&ctx.framework, user.resolved.id).await?,
                    message.channel_id,
                    message.id
                )
            }
        };

        embed = embed.author(EmbedAuthorBuilder::new(slash_author.name).icon_url(ImageSource::url(slash_author.avatar)?));
        embed = embed.field(EmbedFieldBuilder::new("Channel", format!("<#{}>", channel_id)).inline());
        embed = embed.field(EmbedFieldBuilder::new("Message ID", message_id.to_string()).inline());

        ctx.respond(|r| r.add_embed(embed.build()).ephemeral()).await
    }
}
