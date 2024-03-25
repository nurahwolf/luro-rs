use sqlx::types::Json;
use twilight_model::{
    channel::{
        message::{
            sticker::MessageSticker, Component, Embed, Mention, MessageActivity, MessageApplication, MessageFlags, MessageInteraction,
            MessageReference, MessageType, Reaction, RoleSubscriptionData,
        },
        Attachment, Channel, ChannelMention,
    },
    gateway::payload::incoming::MessageUpdate,
    guild::PartialMember,
    id::{
        marker::{ChannelMarker, MessageMarker},
        Id,
    },
    user::User,
    util::Timestamp,
};

use crate::{
    database::sqlx::{Database, Error},
    message::{Message, MessageSource},
};

impl Database {
    pub async fn fetch_message(&self, channel_id: Id<ChannelMarker>, message_id: Id<MessageMarker>) -> Result<Message, Error> {
        #[cfg(feature = "database-sqlx")]
        match fetch_message_sqlx(self, message_id).await {
            Ok(Some(user)) => return Ok(user),
            Ok(None) => tracing::debug!("Message '{message_id}' was not found in the database."),
            Err(why) => tracing::error!("Error raised while trying to find message: {why}"),
        };

        Ok(self.twilight_driver.fetch_message(channel_id, message_id).await?.into())
    }
}

async fn fetch_message_sqlx(db: &Database, message_id: Id<MessageMarker>) -> Result<Option<Message>, Error> {
    let message = sqlx::query_file!("src/database/sqlx/queries/message_fetch.sql", message_id.get() as i64)
        .fetch_optional(&db.pool)
        .await?;

    let message = match message {
        Some(message) => message,
        None => return Ok(None),
    };

    Ok(Some(Message {
        deleted: message.deleted,
        updated_content: message.message_updates.map(|x| x.0),
        source: message.source,
        twilight_message: twilight_model::channel::Message {
            member: message.member.map(|x| x.0),
            activity: message.activity.map(|x| x.0),
            application_id: message.application_id.map(|x| Id::new(x as u64)),
            application: message.application.map(|x| x.0),
            attachments: message.attachments.map(|x| x.0).unwrap_or_default(),
            author: message.author.0.into(),
            channel_id: Id::new(message.channel_id as u64),
            components: message.components.map(|x| x.0).unwrap_or_default(),
            content: message.content.unwrap_or_default(),
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
            referenced_message: None,
            role_subscription_data: message.role_subscription_data.map(|x| x.0),
            sticker_items: message.sticker_items.map(|x| x.0).unwrap_or_default(),
            thread: message.thread.map(|x| x.0),
            timestamp: Timestamp::from_secs(message.timestamp.unix_timestamp()).unwrap(),
            tts: message.tts.unwrap_or_default(),
            webhook_id: message.webhook_id.map(|x| Id::new(x as u64)),
        },
    }))
}
