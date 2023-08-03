use std::convert::TryInto;

use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::Id;
use twilight_util::builder::embed::{EmbedAuthorBuilder, ImageSource, EmbedFieldBuilder};

use crate::models::{LuroSlash, UserData};

use crate::traits::luro_command::LuroCommand;
use crate::traits::luro_functions::LuroFunctions;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "get_message", desc = "Gets a particular message from the cache, or user's data")]
pub struct OwnerGetMessage {
    /// The message ID to get
    message_id: String,
    /// If defined, attempts to find the message from this user's data if not found in the cache
    user: Option<ResolvedUser>
}

#[async_trait]
impl LuroCommand for OwnerGetMessage {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let message_id = Id::new(self.message_id.parse()?);
        let mut embed = ctx.default_embed().await?;
        let ((_, avatar, name), channel_id, message_id) = match ctx.luro.twilight_cache.message(message_id) {
            Some(message) => {
                embed = embed.description(message.content());
                (ctx.fetch_specified_user(&ctx.luro, &message.author()).await?, message.channel_id(), message.id())
            },
            None => {
                let user = match self.user {
                    Some(user) => user,
                    None => return ctx.clone().content("Message not found! Try specifying a user ID if you know who sent it.").ephemeral().respond().await
                };
        
                let user_data = UserData::get_user_settings(&ctx.luro, &user.resolved.id).await?;
                let message = match user_data.messages.get(&message_id) {
                    Some(message) => message,
                    None => return ctx.clone().content("Looks like the user does not have the message ID you provided, sorry.").ephemeral().respond().await
                };
                if let Some(content) = &message.content {
                    embed = embed.description(content)
                }

                let (user, avatar, name) = ctx.get_specified_user(&user, &ctx.interaction);
                ((user.clone(), avatar, name.clone()), message.channel_id, message.id)
            }
        };

        embed = embed.author(EmbedAuthorBuilder::new(name).icon_url(ImageSource::url(avatar)?));
        embed = embed.field(EmbedFieldBuilder::new("Channel", format!("<#{}>", channel_id)).inline());
        embed = embed.field(EmbedFieldBuilder::new("Message ID", message_id.to_string()).inline());

        ctx.embed(embed.build())?.respond().await
    }
}