use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use luro_model::{luro_database_driver::LuroDatabaseDriver, luro_log_channel::LuroLogChannel};
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFieldBuilder, ImageSource, EmbedBuilder};

use crate::{framework::Framework, models::SlashUser, COLOUR_DANGER};
impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn listener_bulk_message_delete(self: &Arc<Self>, mut event: MessageDeleteBulk) -> Result<(), Error> {
        let mut message_authors = HashMap::new();
        let mut embeds = vec![];
        let mut embed = self
            .default_embed(&event.guild_id)
            .await
            .title("Bulk Message Delete")
            .color(COLOUR_DANGER);
        let mut messages = vec![];
        let mut field_count = 0;
        event.ids.reverse();
        for message_id in event.ids {
            if let Some(message) = self.twilight_cache.message(message_id) {
                messages.push(message)
            }
        }

        for message in messages {
            let message_author = message_authors
                .entry(message.author())
                .or_insert(SlashUser::client_fetch(self, event.guild_id, message.author()).await?)
                .clone();

            if field_count >= 25 {
                embeds.push(embed);
                embed = self
                    .default_embed(&event.guild_id)
                    .await
                    .title("Bulk Message Delete")
                    .color(COLOUR_DANGER);
                field_count = 0;
                if message_authors.len() == 1 {
                    let author = message_authors.values().last().unwrap();
                    embed = embed.author(
                        EmbedAuthorBuilder::new(format!("{} - {}", author.name, author.user_id))
                            .icon_url(ImageSource::url(author.avatar.clone())?)
                    )
                }
            }

            field_count += 1;
            if !message.content().is_empty() {
                embed = embed.field(EmbedFieldBuilder::new(
                    format!(
                        "{} - {} - <t:{}:R>",
                        message_author.name,
                        message_author.user_id,
                        message.timestamp().as_secs()
                    ),
                    message.content()
                ))
            }
        }

        if message_authors.len() == 1 {
            let author = message_authors.values().last().unwrap();
            embed = embed.author(
                EmbedAuthorBuilder::new(format!("{} - {}", author.name, author.user_id))
                    .icon_url(ImageSource::url(author.avatar.clone())?)
            )
        }
        embeds.push(embed);

        for mut embed in embeds {
            match field_count != 0 {
                true => self.send_log_channel(&event.guild_id, embed, LuroLogChannel::Message).await?,
                false => {
                    embed = embed.description("Messages were not in my cache! Sorry...");
                    self.send_log_channel(&event.guild_id, embed, LuroLogChannel::Message).await?
                },
            }
        }

        Ok(())
    }
}
