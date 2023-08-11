use luro_model::{
    luro_database_driver::LuroDatabaseDriver, luro_log_channel::LuroLogChannel, luro_message::LuroMessage,
    luro_message_source::LuroMessageSource
};
use regex::Regex;
use std::{fmt::Write, sync::Arc};
use tracing::{debug, info, warn};

use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{framework::Framework, models::SlashUser, COLOUR_DANGER};

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn response_message_modified(self: &Arc<Self>, message: &LuroMessage) -> anyhow::Result<()> {
        debug!(message = ?message, "Message Modified");

        let mut description = String::new();
        let mut embed = self.default_embed(&message.guild_id).await;

        if let Some(author) = message.author.clone() {
            if author.bot {
                debug!("User is a bot");
                return Ok(());
            };
            let slash_user = SlashUser::from(author);
            let embed_author = EmbedAuthorBuilder::new(format!("{} - {}", slash_user.name, slash_user.user_id))
                .icon_url(ImageSource::url(slash_user.avatar)?);
            embed = embed.author(embed_author)
        }

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
                    false => embed = embed.field(EmbedFieldBuilder::new("Original Message", old_message.content()))
                }

                embed = embed.title("Message Edited");

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
                        false => embed = embed.field(EmbedFieldBuilder::new("Updated Message", content))
                    },
                    None => {
                        debug!("No message content, so no need to record it");
                        return Ok(());
                    }
                }
                embed = embed.field(
                    EmbedFieldBuilder::new("Total Edits", format!("Edited `{}` messages!", user_data.message_edits)).inline()
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
                let (_, slash_user) = SlashUser::client_fetch_user(self, old_message.author()).await?;
                let embed_author = EmbedAuthorBuilder::new(slash_user.name).icon_url(ImageSource::url(slash_user.avatar)?);
                embed = embed.author(embed_author).title("Message Deleted").color(COLOUR_DANGER)
            }
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
                    let mut modified_user_data = self.database.get_user(&author.id).await?;

                    // Add the raw message to the user's data
                    modified_user_data.messages.insert(message.id, message.clone());
                    if let Some(ref user) = self.twilight_cache.user(author.id) {
                        modified_user_data.accent_color = user.accent_color;
                        modified_user_data.avatar = user.avatar;
                        modified_user_data.banner = user.banner;
                        modified_user_data.bot = user.bot;
                        modified_user_data.discriminator = Some(user.discriminator().get());
                        modified_user_data.email = user.email.clone();
                        modified_user_data.flags = user.flags;
                        modified_user_data.id = Some(user.id);
                        modified_user_data.locale = user.locale.clone();
                        modified_user_data.mfa_enabled = user.mfa_enabled;
                        modified_user_data.name = Some(user.name.clone());
                        modified_user_data.premium_type = user.premium_type;
                        modified_user_data.public_flags = user.public_flags;
                        modified_user_data.system = user.system;
                        modified_user_data.verified = user.verified;
                    }
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
                    self.database.modify_user(&author.id, &modified_user_data).await?;
                }

                return Ok(());
            }
            LuroMessageSource::None => return Ok(())
        }

        match self.embed_message_modified(message, embed, description).await {
            Ok(embed) => self.send_log_channel(&message.guild_id, embed, LuroLogChannel::Message).await,
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

        embed = embed.field(EmbedFieldBuilder::new("Channel", format!("<#{}>", message.channel_id)).inline());
        embed = embed.field(EmbedFieldBuilder::new("Message ID", message.id.to_string()).inline());
        embed = embed.description(description);
        Ok(embed)
    }
}
