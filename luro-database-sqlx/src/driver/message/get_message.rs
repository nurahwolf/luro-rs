use sqlx::types::Json;
use luro_model::message::{Message, MessageSource};
use sqlx::Error;
use twilight_model::channel::message::sticker::MessageSticker;
use twilight_model::channel::message::Mention;
use twilight_model::channel::Channel;
use twilight_model::channel::{
    message::{
        Component, Embed, MessageActivity, MessageApplication, MessageFlags, MessageInteraction, MessageReference, MessageType, Reaction,
        RoleSubscriptionData,
    },
    Attachment, ChannelMention,
};
use twilight_model::gateway::payload::incoming::MessageUpdate;
use twilight_model::guild::PartialMember;
use twilight_model::id::marker::MessageMarker;
use twilight_model::id::Id;
use twilight_model::user::User;
use twilight_model::util::Timestamp;

use crate::types::DbMessageSource;

impl crate::SQLxDriver {
    pub async fn get_message(&self, message_id: Id<MessageMarker>) -> Result<Option<Message>, Error> {
        let message = sqlx::query_file!(
            "queries/message_fetch.sql",
            message_id.get() as i64
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(match message {
            Some(message) => Some(Message {
                member: message.member.map(|x| x.0),
                activity: message.activity.map(|x| x.0),
                application_id: message.application_id.map(|x| Id::new(x as u64)),
                application: message.application.map(|x| x.0),
                attachments: message.attachments.map(|x| x.0).unwrap_or_default(),
                author: message.author.0,
                channel_id: Id::new(message.channel_id as u64),
                components: message.components.map(|x| x.0).unwrap_or_default(),
                content: message.content.unwrap_or_default(),
                deleted: message.deleted.unwrap_or_default(),
                edited_timestamp: message.edited_timestamp.map(|x| Timestamp::from_secs(x.unix_timestamp()).unwrap()),
                embeds: message.embeds.map(|x| x.0).unwrap_or_default(),
                flags: message.flags.map(|x| x.0),
                guild_id: message.guild_id.map(|x| Id::new(x as u64)),
                id: Id::new(message.message_id as u64),
                interaction: message.interaction.map(|x| x.0),
                kind: message.kind.0,
                mention_channels: message.mention_channels.map(|x| x.0).unwrap_or_default(),
                mention_everyone: message.mention_everyone.unwrap_or_default(),
                mention_roles: message
                    .mention_roles
                    .map(|x| x.into_iter().map(|x| Id::new(x as u64)).collect())
                    .unwrap_or_default(),
                mentions: message.mentions.map(|x| x.0).unwrap_or_default(),
                pinned: message.pinned.unwrap_or_default(),
                reactions: message.reactions.map(|x| x.0).unwrap_or_default(),
                reference: message.reference.map(|x| x.0),
                referenced_message: message.referenced_message.map(|x| x.0),
                role_subscription_data: message.role_subscription_data.map(|x| x.0),
                source: match message.source {
                    DbMessageSource::Message => MessageSource::Message,
                    DbMessageSource::Custom => MessageSource::Custom,
                    DbMessageSource::CachedMessage => MessageSource::CachedMessage,
                    DbMessageSource::MessageUpdate => MessageSource::MessageUpdate,
                    DbMessageSource::MessageDelete => MessageSource::MessageDelete,
                    DbMessageSource::MessageCreate => MessageSource::MessageCreate,
                    DbMessageSource::None => MessageSource::None,
                },
                sticker_items: message.sticker_items.map(|x| x.0).unwrap_or_default(),
                thread: message.thread.map(|x| x.0),
                timestamp: Timestamp::from_secs(message.timestamp.unix_timestamp()).unwrap(),
                tts: message.tts.unwrap_or_default(),
                updated_content: None,
                webhook_id: message.webhook_id.map(|x| Id::new(x as u64)),
            }),
            None => None,
        })
    }
}