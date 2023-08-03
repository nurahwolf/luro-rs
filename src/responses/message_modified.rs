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
        debug!(message = ?message, "Message Modified");

        let mut description = String::new();
        let mut embed = self.default_embed(&message.guild_id);

        if let Some(author) = message.author.clone() {
            if author.bot {
                debug!("User is a bot");
                return Ok(());
            };
            let (avatar, name, _author) = self.formatted_user(author);
            let embed_author = EmbedAuthorBuilder::new(name).icon_url(ImageSource::url(avatar)?);
            embed = embed.author(embed_author)
        }

        match message.source {
            LuroMessageSource::MessageUpdate => {
                let old_message = match self.twilight_cache.message(message.id) {
                    Some(old_message) => old_message,
                    None => {
                        info!("Old message does not exist in the cache");
                        return Ok(());
                    },
                };
                embed = embed.title("Message Edited");
                match &message.content {
                    Some(content) => {
                        writeln!(description, "**Original Message:**\n{}\n\n", old_message.content())?;
                        writeln!(description, "**Updated Message:**\n{content}")?
                    },
                    None => {
                        debug!("No message content, so no need to record it");
                        return Ok(())
                    }
                }
            }
            LuroMessageSource::MessageDelete => {
                let old_message = match self.twilight_cache.message(message.id) {
                    Some(old_message) => old_message,
                    None => {
                        info!("Old message does not exist in the cache");
                        return Ok(());
                    },
                };
                writeln!(description, "**Original Message:**\n{}\n\n", old_message.content())?;
                let (_author, avatar, name) = self.fetch_specified_user(self, &old_message.author()).await?;
                let embed_author = EmbedAuthorBuilder::new(name).icon_url(ImageSource::url(avatar)?);
                embed = embed.author(embed_author).title("Message Deleted").color(COLOUR_DANGER)
            },
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

                return Ok(())
            }
            LuroMessageSource::None => return Ok(())
        }

        match self.embed_message_modified(message, embed, description).await {
            Ok(embed) => self.send_log_channel(&message.guild_id, embed).await,
            Err(why) => {
                info!(why = ?why, "Failed to send to guild log channel");
                Ok(())
            }
        }
    }

    /// Create an embed that details a modified message
    pub async fn embed_message_modified(self: &Arc<Self>, message: &LuroMessage, mut embed: EmbedBuilder, mut description: String) -> anyhow::Result<EmbedBuilder> {
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
