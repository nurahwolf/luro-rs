use anyhow::anyhow;
use std::{fmt::Write, sync::Arc};
use tracing::{debug, info};

use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{
    models::{LuroFramework, LuroMessage, LuroMessageSource, UserData},
    traits::luro_functions::LuroFunctions,
    COLOUR_DANGER
};

impl LuroFramework {
    pub async fn response_message_modified(self: &Arc<Self>, message: &LuroMessage) -> anyhow::Result<()> {
        match self.embed_message_modified(message).await {
            Ok(embed) => self.send_log_channel(&message.guild_id, embed).await,
            Err(why) => {
                info!(why = ?why, "Handling failed message modified handler");
                Ok(())
            }
        }
    }

    /// Create an embed that details a modified message
    pub async fn embed_message_modified(self: &Arc<Self>, message: &LuroMessage) -> anyhow::Result<EmbedBuilder> {
        debug!(message = ?message, "Message Modified");
        let (mut embed, old_message) = match self.twilight_cache.message(message.id) {
            Some(old_message) => (self.default_embed(&message.guild_id), old_message),
            None => {
                return Err(anyhow!("Failed to find old message in cache!"));
            }
        };

        if let Some(author) = message.author.clone() {
            if author.bot {
                return Err(anyhow!("User is a bot"));
            };
            let (avatar, name, _author) = self.formatted_user(author);
            let embed_author = EmbedAuthorBuilder::new(name).icon_url(ImageSource::url(avatar)?);
            embed = embed.author(embed_author)
        }

        let mut description = format!("**Original Message:**\n{}\n\n", old_message.content());
        match message.source {
            LuroMessageSource::MessageUpdate => {
                embed = embed.title("Message Edited");
                match &message.content {
                    Some(content) => writeln!(description, "**Updated Message:**\n{content}")?,
                    None => return Err(anyhow!("No message content found"))
                }
            }
            LuroMessageSource::MessageDelete => embed = embed.title("Message Deleted").color(COLOUR_DANGER),
            LuroMessageSource::MessageCreate => {
                let mut content = String::new();
                if let Some(embeds) = &message.embeds {
                    for embed in embeds {
                        if let Some(ref description) = embed.description {
                            content.push_str(description)
                        }
                    }
                }

                if let Some(message_content) = &message.content {
                    content.push_str(message_content)
                }

                if let Some(author) = &message.author && !content.is_empty() {
                    UserData::write_words(self, &content, &author.id, message).await?;
                }
            }
            LuroMessageSource::None => return Err(anyhow!("No message type"))
        }

        match message.guild_id {
            Some(guild_id) => {
                embed = embed.url(format!(
                    "https://discord.com/channels/{guild_id}/{}/{}",
                    message.channel_id, message.id
                ))
            }
            None => {
                embed = embed.url(format!(
                    "https://discord.com/channels/@me/{}/{}",
                    message.channel_id, message.id
                ))
            }
        }

        if description.len() > 4096 {
            description.truncate(4093);
            description.push_str("...")
        }

        embed = embed.field(EmbedFieldBuilder::new("Channel", format!("<#{}>", message.channel_id)).inline());
        embed = embed.field(EmbedFieldBuilder::new("Message ID", message.id.to_string()).inline());
        embed = embed.description(description);
        Ok(embed)
    }
}
