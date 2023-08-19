use luro_builder::embed::EmbedBuilder;
use luro_model::{
    constants::COLOUR_DANGER, luro_database_driver::LuroDatabaseDriver, luro_log_channel::LuroLogChannel,
    luro_message::LuroMessage, luro_message_source::LuroMessageSource
};
use regex::Regex;
use std::{fmt::Write, sync::Arc};
use tracing::{debug, info, trace, warn};

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn response_message_modified(self: &Arc<Self>, message: &LuroMessage) -> anyhow::Result<()> {
        trace!(message = ?message, "Message Modified");

        let mut description = String::new();
        let mut embed = self.default_embed(&message.guild_id).await;

        if message.user.bot {
            debug!("User is a bot");
            return Ok(());
        };

        embed.author(|author| {
            author
                .name(message.user.name())
                .icon_url(message.user.avatar())
                .url(message.link())
        });

        match message.source {
            LuroMessageSource::MessageUpdate => {
                let old_message = match self.twilight_cache.message(message.id) {
                    Some(old_message) => old_message,
                    None => {
                        warn!("Old message does not exist in the cache");
                        return Ok(());
                    }
                };
                match old_message.content().len() > 1024 {
                    true => writeln!(description, "**Original Message:**\n{}\n", old_message.content())?,
                    false => {
                        embed.create_field("Original Message", old_message.content(), false);
                    }
                };

                embed.title("Message Edited");

                let user_data;
                {
                    let mut data = self.database.get_user(&old_message.author()).await?;
                    data.message_edits += 1;
                    self.database.modify_user(&old_message.author(), &data).await?;
                    user_data = data
                }
                match &message.content {
                    Some(content) => match content.len() > 1024 {
                        true => writeln!(description, "**Updated Message:**\n{content}")?,
                        false => {
                            embed.create_field("Updated Message", content, false);
                        }
                    },
                    None => {
                        debug!("No message content, so no need to record it");
                        return Ok(());
                    }
                }
                embed.create_field(
                    "Total Edits",
                    &format!("Edited `{}` messages!", user_data.message_edits),
                    true
                );
            }
            LuroMessageSource::MessageDelete => {
                let old_message = match self.twilight_cache.message(message.id) {
                    Some(old_message) => old_message,
                    None => {
                        warn!("Old message does not exist in the cache");
                        return Ok(());
                    }
                };
                if old_message.content().is_empty() {
                    return Ok(());
                }
                writeln!(description, "**Original Message:**\n{}\n\n", old_message.content())?;
                embed
                    .author(|author| author.name(message.user.name()).icon_url(message.user.avatar()))
                    .colour(COLOUR_DANGER);
            }
            LuroMessageSource::MessageCreate => {
                let mut content = String::new();
                for embed in &message.embeds {
                    if let Some(ref description) = embed.description {
                        content.push_str(description)
                    }
                }

                if let Some(message_content) = &message.content {
                    content.push_str(message_content)
                }

                if !content.is_empty() {
                    let mut modified_user_data = self.database.get_user(&message.author_id.unwrap()).await?;
                    modified_user_data.global_name = message.user.global_name.clone();
                    modified_user_data.messages.insert(message.id, message.clone());
                    modified_user_data.update_lurouser(&message.user);

                    // First perform analysis
                    let regex = Regex::new(r"\b[\w-]+\b").unwrap();
                    for capture in regex.captures_iter(&content) {
                        let word = match capture.get(0) {
                            Some(word) => word.as_str().to_ascii_lowercase(),
                            None => "".to_owned()
                        };
                        let size = word.len();
                        modified_user_data.wordcount += 1;
                        modified_user_data.averagesize += size;
                        *modified_user_data.words.entry(word).or_insert(0) += 1;
                        *modified_user_data.wordsize.entry(size).or_insert(0) += 1;
                    }

                    // Save
                    self.database
                        .modify_user(&message.author_id.unwrap(), &modified_user_data)
                        .await?;
                }

                return Ok(());
            }
            _ => return Ok(())
        }

        match self.embed_message_modified(message, embed, description).await {
            Ok(embed) => {
                self.send_log_channel(&message.guild_id, embed.into(), LuroLogChannel::Message)
                    .await
            }
            Err(why) => {
                info!(why = ?why, "Failed to send to guild log channel");
                Ok(())
            }
        }
    }

    /// Create an embed that details a modified message
    pub async fn embed_message_modified(
        self: &Arc<Self>,
        message: &LuroMessage,
        mut embed: EmbedBuilder,
        description: String
    ) -> anyhow::Result<EmbedBuilder> {
        match message.guild_id {
            Some(guild_id) => {
                embed.url(format!(
                    "https://discord.com/channels/{guild_id}/{}/{}",
                    message.channel_id, message.id
                ));
            }
            None => {
                embed.url(format!(
                    "https://discord.com/channels/@me/{}/{}",
                    message.channel_id, message.id
                ));
            }
        }
        embed.create_field("Channel", &format!("<#{}>", message.channel_id), true);
        embed.create_field("Message ID", &message.id.to_string(), true);
        embed.description(description);
        Ok(embed)
    }
}
