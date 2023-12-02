use futures_util::TryStreamExt;
use luro_model::types::{Message, MessageData, MessageSource, Quote};
use sqlx::types::Json;
use twilight_model::channel::message::sticker::MessageSticker;
use twilight_model::channel::message::Component;
use twilight_model::channel::message::Embed;
use twilight_model::channel::message::Mention;
use twilight_model::channel::message::MessageActivity;
use twilight_model::channel::message::MessageApplication;
use twilight_model::channel::message::MessageFlags;
use twilight_model::channel::message::MessageInteraction;
use twilight_model::channel::message::MessageReference;
use twilight_model::channel::message::MessageType;
use twilight_model::channel::message::Reaction;
use twilight_model::channel::message::RoleSubscriptionData;
use twilight_model::channel::Attachment;
use twilight_model::channel::Channel;
use twilight_model::channel::ChannelMention;
use twilight_model::gateway::payload::incoming::MessageUpdate;
use twilight_model::guild::PartialMember;
use twilight_model::user::User;
use twilight_model::{id::Id, util::Timestamp};

use crate::types::DbMessageSource;

impl crate::SQLxDriver {
    /// Add a quote to the database, returning the added quote ID
    pub async fn quotes_fetch(&self) -> anyhow::Result<(Vec<Quote>, Vec<Quote>)> {
        let mut sfw_quotes = vec![];
        let mut nsfw_quotes = vec![];
        let mut query = sqlx::query_file!("queries/quotes/quotes_fetch.sql").fetch(&self.pool);

        loop {
            let quote = match query.try_next().await {
                Ok(Some(quote)) => quote,
                Ok(None) => return Ok((sfw_quotes, nsfw_quotes)),
                Err(why) => {
                    tracing::error!("DB Error during loop: {why:?}");
                    continue;
                }
            };

            let quote = Quote {
                channel_id: Id::new(quote.channel_id as u64),
                message: Message {
                    data: Some(MessageData {
                        deleted: quote.deleted.unwrap_or_default(),
                        updated_content: quote.message_updates.map(|x| x.0),
                    }),
                    member: quote.member.map(|x| x.0),
                    activity: quote.activity.map(|x| x.0),
                    application_id: quote.application_id.map(|x| Id::new(x as u64)),
                    application: quote.application.map(|x| x.0),
                    attachments: quote.attachments.map(|x| x.0).unwrap_or_default(),
                    author: quote.author.0.into(),
                    channel_id: Id::new(quote.channel_id as u64),
                    components: quote.components.map(|x| x.0).unwrap_or_default(),
                    content: quote.content.unwrap_or_default(),
                    edited_timestamp: quote.edited_timestamp.map(|x| Timestamp::from_secs(x.unix_timestamp()).unwrap()),
                    embeds: quote.embeds.map(|x| x.0).unwrap_or_default(),
                    flags: quote.flags.map(|x| x.0),
                    guild_id: quote.guild_id.map(|x| Id::new(x as u64)),
                    id: Id::new(quote.message_id as u64),
                    interaction: quote.interaction.map(|x| x.0),
                    kind: quote.kind.0,
                    mention_channels: quote.mention_channels.map(|x| x.0).unwrap_or_default(),
                    mention_everyone: quote.mention_everyone.unwrap_or_default(),
                    mention_roles: quote
                        .mention_roles
                        .map(|x| x.into_iter().map(|x| Id::new(x as u64)).collect())
                        .unwrap_or_default(),
                    mentions: quote.mentions.map(|x| x.0).unwrap_or_default(),
                    pinned: quote.pinned.unwrap_or_default(),
                    reactions: quote.reactions.map(|x| x.0).unwrap_or_default(),
                    reference: quote.reference.map(|x| x.0),
                    referenced_message: quote.referenced_message.map(|x| x.0),
                    role_subscription_data: quote.role_subscription_data.map(|x| x.0),
                    source: match quote.source {
                        DbMessageSource::Message => MessageSource::TwilightMessage,
                        DbMessageSource::Custom => MessageSource::Custom,
                        DbMessageSource::CachedMessage => MessageSource::CachedMessage,
                        DbMessageSource::MessageUpdate => MessageSource::MessageUpdate,
                        DbMessageSource::MessageDelete => MessageSource::MessageDelete,
                        DbMessageSource::MessageCreate => MessageSource::MessageCreate,
                        DbMessageSource::None => MessageSource::None,
                    },
                    sticker_items: quote.sticker_items.map(|x| x.0).unwrap_or_default(),
                    thread: quote.thread.map(|x| x.0),
                    timestamp: Timestamp::from_secs(quote.timestamp.unix_timestamp()).unwrap(),
                    tts: quote.tts.unwrap_or_default(),
                    webhook_id: quote.webhook_id.map(|x| Id::new(x as u64)),
                },
                nsfw: quote.nsfw,
                quote_id: quote.id,
            };

            match quote.nsfw {
                true => nsfw_quotes.push(quote),
                false => sfw_quotes.push(quote),
            }
        }
    }
}
