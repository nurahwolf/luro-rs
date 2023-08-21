use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use luro_builder::{embed::EmbedBuilder, response::LuroResponse};
use luro_model::{database::drivers::LuroDatabaseDriver, guild::log_channel::LuroLogChannel};
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

use crate::{framework::Framework, COLOUR_DANGER};
impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn listener_bulk_message_delete(self: &Arc<Self>, mut event: MessageDeleteBulk) -> Result<(), Error> {
        // Sort message IDs from oldest to newest, then fetch the messages from cache.
        let mut errors = 0;
        let mut messages = vec![];

        event.ids.reverse();
        for message_id in event.ids {
            match self.twilight_cache.message(message_id) {
                Some(message) => messages.push(message),
                None => errors += 1
            }
        }

        // Now loop through those messages
        let mut embed = EmbedBuilder::default();
        embed.title("Bulk Message Delete").colour(COLOUR_DANGER);
        let mut response = LuroResponse::default();
        let mut message_authors = HashMap::new();
        let mut field_count = 0;
        for message in messages {
            // Save each author in a hash map. If there is only one author per embed, then set them as the embed author.
            let message_author = message_authors
                .entry(message.author())
                .or_insert(self.database.get_user(&message.author(), &self.twilight_client).await?)
                .clone();

            // We hit the 25 field cap per embed. Let's roll this embed up and start again.
            if field_count >= 25 {
                if let Some(audit_author) = message_authors.values().last() {
                    embed.author(|author| {
                        author
                            .name(format!("{} - {}", audit_author.name, audit_author.id))
                            .icon_url(audit_author.avatar())
                    });
                }
                message_authors.clear();
                response.add_embed(embed.clone());
                embed.set_fields(vec![]); // This sets the builder to have no fields
                field_count = 0;
            }

            // Now that we checked to make sure we have space for more fields, add another...
            field_count += 1;
            if !message.content().is_empty() {
                embed.field(|field| {
                    field.value(format!(
                        "<@{}> - <#{}> - <t:{}:R> - `{}`\n{}",
                        message_author.id,
                        message.channel_id(),
                        message.timestamp().as_secs(),
                        message.id(),
                        message.content(),
                    ))
                });
            }
        }

        // If our embed has only one author, set them as the embed author
        if let Some(audit_author) = message_authors.values().last() && message_authors.len() == 1 {
            embed.author(|author| author.name(format!("{} - {}", audit_author.name(), audit_author.id)).icon_url(audit_author.avatar()));
        }

        // If no fields were added, that means we were not able to get any messages
        if field_count == 1 {
            embed.description("Messages were not in my cache so I can't show what was deleted! Sorry...");
        }

        // Add a footer if we had some errors... Only on the last embed
        if errors != 0 {
            embed.footer(|footer| footer.text(format!("Failed to fetch {errors} messages! Sorry...")));
        }

        // Add our crafted embed to the response
        response.embed(|e| {
            *e = embed;
            e
        });

        // Send out response
        self.send_log_channel_new(&event.guild_id, LuroLogChannel::Message, |r| {
            *r = response;
            r
        })
        .await
    }
}
