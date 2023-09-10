use luro_builder::embed::EmbedBuilder;
use luro_model::{
    database_driver::LuroDatabaseDriver,
    guild::log_channel::LuroLogChannel,
    message::{LuroMessage, LuroMessageSource},
    COLOUR_DANGER,
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
        let mut user = self.database.get_user(&message.author).await?;

        if user.bot {
            debug!("User is a bot");
            return Ok(());
        };

        if message.webhook_id.is_some() {
            debug!("Message was sent by a webhook");
            return Ok(());
        }

        embed.author(|author| author.name(user.name()).icon_url(user.avatar()).url(message.link()));

        match message.source {
            LuroMessageSource::MessageUpdate => {
                let user_data = match self.database.get_user(&message.author).await {
                    Ok(data) => Some(data),
                    Err(why) => {
                        warn!(why = ?why, "Could not fetch user data!");
                        None
                    }
                };

                let mut old_message = self
                    .twilight_cache
                    .message(message.id)
                    .map(|data| LuroMessage::from(data.clone()));

                if old_message.is_none() && let Some(ref user_data) = user_data {
                    old_message = user_data.messages.get(&message.id).cloned();
                }

                let message = match old_message {
                    Some(message) => message,
                    None => {
                        warn!("Old message does not exist in the cache and not in the sender's data!");
                        return Ok(());
                    }
                };

                embed.title("Message Edited");
                match message.content.len() > 1024 {
                    true => writeln!(description, "**Original Message:**\n{}\n", &message.content)?,
                    false => {
                        embed.create_field("Original Message", &message.content, false);
                    }
                };

                if let Some(mut data) = user_data {
                    data.message_edits += 1;
                    embed.create_field("Total Edits", &format!("Edited `{}` messages!", &data.message_edits), true);
                    self.database.modify_user(&message.author, &data).await?;
                }

                match !message.content.is_empty() {
                    true => match message.content.len() > 1024 {
                        true => writeln!(description, "**Updated Message:**\n{}", message.content)?,
                        false => {
                            embed.create_field("Updated Message", &message.content, false);
                        }
                    },
                    false => {
                        debug!("No message content, so no need to record it");
                        return Ok(());
                    }
                }
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
                    .author(|author| author.name(user.name()).icon_url(user.avatar()))
                    .colour(COLOUR_DANGER);
            }
            LuroMessageSource::MessageCreate => {
                let mut content = String::new();
                for embed in &message.embeds {
                    if let Some(ref description) = embed.description {
                        content.push_str(description)
                    }
                }

                if !message.content.is_empty() {
                    content.push_str(&message.content)
                }

                if !content.is_empty() {
                    user.messages.insert(message.id, message.clone());

                    // First perform analysis
                    let regex = Regex::new(r"\b[\w-]+\b").unwrap();
                    for capture in regex.captures_iter(&content) {
                        let word = match capture.get(0) {
                            Some(word) => word.as_str().to_ascii_lowercase(),
                            None => "".to_owned(),
                        };
                        let size = word.len();
                        user.wordcount += 1;
                        user.averagesize += size;
                        *user.words.entry(word).or_insert(0) += 1;
                        *user.wordsize.entry(size).or_insert(0) += 1;
                    }

                    // Save
                    self.database.modify_user(&message.author, &user).await?;
                }

                return Ok(());
            }
            _ => return Ok(()),
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
        description: String,
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
