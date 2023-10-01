use anyhow::anyhow;
use std::collections::HashMap;

use futures_util::TryStreamExt;
use luro_model::message::{LuroMessage, LuroMessageSource, LuroMessageSourceV2};
use sqlx::types::Json;
use sqlx::Error;
use time::OffsetDateTime;
use tracing::debug;
use twilight_model::channel::Channel;
use twilight_model::guild::PartialMember;
use twilight_model::channel::message::sticker::MessageSticker;
use twilight_model::channel::message::Mention;
use twilight_model::gateway::payload::incoming::{MessageCreate, MessageDelete, MessageUpdate};
use twilight_model::user::User;
use twilight_model::{
    channel::{
        message::{
            Component, Embed, MessageActivity, MessageApplication, MessageFlags, MessageInteraction, MessageReference,
            MessageType, Reaction, RoleSubscriptionData,
        },
        Attachment, ChannelMention, Message,
    },
    id::{marker::MessageMarker, Id},
    util::Timestamp,
};

use crate::{DatabaseMessage, DatabaseMessageSource, LuroDatabase};

mod handle_message_delete_bulk;



impl From<LuroMessage> for DatabaseMessage {
    fn from(message: LuroMessage) -> Self {
        Self {
            member: message.member.map(Json),
            activity: message.activity.map(|x| x.into()),
            application_id: message.application_id.map(|x| x.get() as i64),
            application: message.application.map(|x| x.into()),
            attachments: Some(message.attachments.into()),
            author: Json(message.author),
            channel_id: message.channel_id.get() as i64,
            components: Some(message.components.into()),
            content: message.content.into(),
            deleted: message.deleted.into(),
            edited_timestamp: message
                .edited_timestamp
                .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
            embeds: Some(message.embeds.into()),
            flags: message.flags.map(|x| x.into()),
            guild_id: message.guild_id.map(|x| x.get() as i64),
            id: message.id.get() as i64,
            interaction: message.interaction.map(|x| x.into()),
            kind: Json(message.kind),
            mention_channels: Some(message.mention_channels.into()),
            mention_everyone: message.mention_everyone.into(),
            mention_roles: message
                .mention_roles
                .into_iter()
                .map(|x| x.get() as i64)
                .collect::<Vec<_>>()
                .into(),
            mentions: match !message.mentions.is_empty() {
                true => None,
                false => Some(Json(message.mentions)),
            },
            pinned: message.pinned.into(),
            reactions: Some(Json(message.reactions)),
            reference: message.reference.map(|x| x.into()),
            referenced_message: message.referenced_message.map(|x| x.into()),
            role_subscription_data: message.role_subscription_data.map(|x| x.into()),
            source: match message.source {
                LuroMessageSource::Message => DatabaseMessageSource::Message,
                LuroMessageSource::Custom => DatabaseMessageSource::Custom,
                LuroMessageSource::CachedMessage => DatabaseMessageSource::CachedMessage,
                LuroMessageSource::MessageUpdate => DatabaseMessageSource::MessageUpdate,
                LuroMessageSource::MessageDelete => DatabaseMessageSource::MessageDelete,
                LuroMessageSource::MessageCreate => DatabaseMessageSource::MessageCreate,
                LuroMessageSource::None => DatabaseMessageSource::None,
            },
            sticker_items: Some(Json(message.sticker_items)),
            thread: message.thread.map(Json),
            timestamp: OffsetDateTime::from_unix_timestamp(message.timestamp.as_secs()).unwrap(),
            tts: Some(message.tts),
            message_updates: None,
            webhook_id: message.webhook_id.map(|x| x.get() as i64),
        }
    }
}

impl From<DatabaseMessage> for LuroMessage {
    fn from(message: DatabaseMessage) -> Self {
        Self {
            member: message.member.map(|x|x.0),
            activity: message.activity.map(|x| x.0),
            application_id: message.application_id.map(|x| Id::new(x as u64)),
            application: message.application.map(|x| x.0),
            attachments: message.attachments.map(|x| x.0).unwrap_or_default(),
            author: message.author.0,
            channel_id: Id::new(message.channel_id as u64),
            components: message.components.map(|x| x.0).unwrap_or_default(),
            content: message.content.unwrap_or_default(),
            deleted: message.deleted.unwrap_or_default(),
            edited_timestamp: message
                .edited_timestamp
                .map(|x| Timestamp::from_secs(x.unix_timestamp()).unwrap()),
            embeds: message.embeds.map(|x| x.0).unwrap_or_default(),
            flags: message.flags.map(|x| x.0),
            guild_id: message.guild_id.map(|x| Id::new(x as u64)),
            id: Id::new(message.id as u64),
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
                DatabaseMessageSource::Message => LuroMessageSource::Message,
                DatabaseMessageSource::Custom => LuroMessageSource::Custom,
                DatabaseMessageSource::CachedMessage => LuroMessageSource::CachedMessage,
                DatabaseMessageSource::MessageUpdate => LuroMessageSource::MessageUpdate,
                DatabaseMessageSource::MessageDelete => LuroMessageSource::MessageDelete,
                DatabaseMessageSource::MessageCreate => LuroMessageSource::MessageCreate,
                DatabaseMessageSource::None => LuroMessageSource::None,
            },
            sticker_items: message.sticker_items.map(|x| x.0).unwrap_or_default(),
            thread: message.thread.map(|x| x.0),
            timestamp: Timestamp::from_secs(message.timestamp.unix_timestamp()).unwrap(),
            tts: message.tts.unwrap_or_default(),
            updated_content: None,
            webhook_id: message.webhook_id.map(|x| Id::new(x as u64)),
        }
    }
}

impl LuroDatabase {
    pub async fn get_message(&self, id: i64) -> Result<Option<LuroMessage>, Error> {
        let query = sqlx::query_as!(
            DatabaseMessage,
            "SELECT 
                activity as \"activity: Json<MessageActivity>\",
                application_id,
                application as \"application: Json<MessageApplication>\",
                attachments as \"attachments: Json<Vec<Attachment>>\",
                author as \"author: Json<User>\",
                channel_id,
                components as \"components: Json<Vec<Component>>\",
                content,
                deleted,
                edited_timestamp,
                embeds as \"embeds: Json<Vec<Embed>>\",
                flags as \"flags: Json<MessageFlags>\",
                guild_id,
                id,
                interaction as \"interaction: Json<MessageInteraction>\",
                kind as \"kind: Json<MessageType>\", 
                mention_channels as \"mention_channels: Json<Vec<ChannelMention>>\",
                mention_everyone,
                mention_roles as \"mention_roles: Vec<i64>\",
                mentions as \"mentions: Json<Vec<Mention>>\",
                pinned,
                reactions as \"reactions: Json<Vec<Reaction>>\",
                reference as \"reference: Json<MessageReference>\",
                referenced_message as \"referenced_message: Json<Box<Message>>\",
                role_subscription_data as \"role_subscription_data: Json<RoleSubscriptionData>\",
                source as \"source: DatabaseMessageSource\",
                sticker_items as \"sticker_items: Json<Vec<MessageSticker>>\",
                thread as \"thread: Json<Channel>\",
                member as \"member: Json<PartialMember>\",
                timestamp,
                tts,
                webhook_id,
                message_updates as \"message_updates: Json<Vec<MessageUpdate>>\"
            FROM messages WHERE id = $1",
            id
        );

        query.fetch_optional(&self.0).await.map(|x| x.map(|x| x.into()))
    }

    pub async fn get_messages(&self) -> HashMap<Id<MessageMarker>, LuroMessage> {
        let mut messages = HashMap::new();
        let mut query = sqlx::query_as!(
            DatabaseMessage,
            "SELECT 
                activity as \"activity: Json<MessageActivity>\",
                application_id,
                application as \"application: Json<MessageApplication>\",
                attachments as \"attachments: Json<Vec<Attachment>>\",
                author as \"author: Json<User>\",
                channel_id,
                components as \"components: Json<Vec<Component>>\",
                content,
                deleted,
                edited_timestamp,
                embeds as \"embeds: Json<Vec<Embed>>\",
                flags as \"flags: Json<MessageFlags>\",
                guild_id,
                id,
                interaction as \"interaction: Json<MessageInteraction>\",
                kind as \"kind: Json<MessageType>\", 
                mention_channels as \"mention_channels: Json<Vec<ChannelMention>>\",
                mention_everyone,
                mention_roles as \"mention_roles: Vec<i64>\",
                mentions as \"mentions: Json<Vec<Mention>>\",
                pinned,
                reactions as \"reactions: Json<Vec<Reaction>>\",
                reference as \"reference: Json<MessageReference>\",
                referenced_message as \"referenced_message: Json<Box<Message>>\",
                role_subscription_data as \"role_subscription_data: Json<RoleSubscriptionData>\",
                source as \"source: DatabaseMessageSource\",
                sticker_items as \"sticker_items: Json<Vec<MessageSticker>>\",
                thread as \"thread: Json<Channel>\",
                timestamp,
                member as \"member: Json<PartialMember>\",
                tts,
                webhook_id,
                message_updates as \"message_updates: Json<Vec<MessageUpdate>>\"
            FROM messages"
        )
        .fetch(&self.0);

        while let Ok(Some(message)) = query.try_next().await {
            messages.insert(Id::new(message.author.id.get()), message.into());
        }

        messages
    }

    pub async fn update_message(&self, message: LuroMessageSourceV2) -> anyhow::Result<Option<LuroMessage>> {
        Ok(match message {
            LuroMessageSourceV2::CachedMessage(message) => todo!(),
            LuroMessageSourceV2::Custom(message) => self.handle_luro_message(message).await?,
            LuroMessageSourceV2::Message(message) => todo!(),
            LuroMessageSourceV2::MessageCreate(message) => self.handle_message_create(message).await?,
            LuroMessageSourceV2::MessageDelete(message) => self.handle_message_delete(message).await?,
            LuroMessageSourceV2::MessageDeleteBulk(messages) => self.handle_message_delete_bulk(messages).await?,
            LuroMessageSourceV2::MessageUpdate(message) => self.handle_message_update(message).await?,
            LuroMessageSourceV2::None => return Err(anyhow!("No message data passed!")),
        })
    }

    async fn handle_message_update(&self, message: MessageUpdate) -> Result<Option<LuroMessage>, Error> {
        debug!("Handling message_create {:#?}", message);

        let query = sqlx::query_as!(
                DatabaseMessage,
                "UPDATE messages
                SET message_updates = message_updates || $2
                WHERE id = $1
                RETURNING
                    activity as \"activity: Json<MessageActivity>\",
                    application_id,
                    application as \"application: Json<MessageApplication>\",
                    attachments as \"attachments: Json<Vec<Attachment>>\",
                    author as \"author: Json<User>\",
                    channel_id,
                    components as \"components: Json<Vec<Component>>\",
                    content,
                    deleted,
                    edited_timestamp,
                    embeds as \"embeds: Json<Vec<Embed>>\",
                    flags as \"flags: Json<MessageFlags>\",
                    guild_id,
                    id,
                    interaction as \"interaction: Json<MessageInteraction>\",
                    kind as \"kind: Json<MessageType>\", 
                    mention_channels as \"mention_channels: Json<Vec<ChannelMention>>\",
                    mention_everyone,
                    mention_roles as \"mention_roles: Vec<i64>\",
                    mentions as \"mentions: Json<Vec<Mention>>\",
                    pinned,
                    reactions as \"reactions: Json<Vec<Reaction>>\",
                    reference as \"reference: Json<MessageReference>\",
                    referenced_message as \"referenced_message: Json<Box<Message>>\",
                    role_subscription_data as \"role_subscription_data: Json<RoleSubscriptionData>\",
                    source as \"source: DatabaseMessageSource\",
                    sticker_items as \"sticker_items: Json<Vec<MessageSticker>>\",
                    thread as \"thread: Json<Channel>\",
                    member as \"member: Json<PartialMember>\",
                    timestamp,
                    tts,
                    webhook_id,
                    message_updates as \"message_updates: Json<Vec<MessageUpdate>>\"
            ",
                message.id.get() as i64,
                Json(message) as _,
            );

            query.fetch_optional(&self.0).await.map(|x| x.map(|x|x.into()))
        }

    async fn handle_message_create(&self, message: MessageCreate) -> Result<Option<LuroMessage>, Error> {
        debug!("Handling message_create {:#?}", message);

        let query = sqlx::query_as!(
                DatabaseMessage,
                "INSERT INTO messages (
                    activity,
                    application_id,
                    application,
                    attachments,
                    author,
                    channel_id,
                    components,
                content,
                deleted,
                edited_timestamp,
                embeds,
                flags,
                guild_id,
                id,
                interaction,
                kind, 
                mention_channels,
                mention_everyone,
                mention_roles,
                mentions,
                pinned,
                reactions,
                reference,
                referenced_message,
                role_subscription_data,
                source,
                sticker_items,
                thread,
                timestamp,
                tts,
                webhook_id,
                member
            ) VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11 , $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32)
            ON CONFLICT (id)
            DO UPDATE SET
                activity = $1,
                application_id = $2,
                application = $3,
                attachments = $4,
                author = $5,
                channel_id = $6,
                components = $7,
                content = $8,
                deleted = $9,
                edited_timestamp = $10,
                embeds = $11,
                flags = $12,
                guild_id = $13,
                id = $14,
                interaction = $15,
                kind = $16,
                mention_channels = $17,
                mention_everyone = $18,
                mention_roles = $19,
                mentions = $20,
                pinned = $21,
                reactions = $22,
                reference = $23,
                referenced_message = $24,
                role_subscription_data = $25,
                source = $26,
                sticker_items = $27,
                thread = $28,
                timestamp = $29,
                tts = $30,
                webhook_id = $31,
                member = $32
            RETURNING
                activity as \"activity: Json<MessageActivity>\",
                application_id,
                application as \"application: Json<MessageApplication>\",
                attachments as \"attachments: Json<Vec<Attachment>>\",
                author as \"author: Json<User>\",
                channel_id,
                components as \"components: Json<Vec<Component>>\",
                content,
                deleted,
                edited_timestamp,
                embeds as \"embeds: Json<Vec<Embed>>\",
                flags as \"flags: Json<MessageFlags>\",
                guild_id,
                id,
                interaction as \"interaction: Json<MessageInteraction>\",
                kind as \"kind: Json<MessageType>\", 
                mention_channels as \"mention_channels: Json<Vec<ChannelMention>>\",
                mention_everyone,
                mention_roles as \"mention_roles: Vec<i64>\",
                mentions as \"mentions: Json<Vec<Mention>>\",
                pinned,
                reactions as \"reactions: Json<Vec<Reaction>>\",
                reference as \"reference: Json<MessageReference>\",
                referenced_message as \"referenced_message: Json<Box<Message>>\",
                role_subscription_data as \"role_subscription_data: Json<RoleSubscriptionData>\",
                source as \"source: DatabaseMessageSource\",
                sticker_items as \"sticker_items: Json<Vec<MessageSticker>>\",
                thread as \"thread: Json<Channel>\",
                member as \"member: Json<PartialMember>\",
                timestamp,
                tts,
                webhook_id,
                message_updates as \"message_updates: Json<Vec<MessageUpdate>>\"
            ",
                message.activity.clone().map(|x|Json(x)) as _,
                message.application_id.map(|x|x.get() as i64),
                message.application.clone().map(|x|Json(x)) as _,
                match message.attachments.is_empty() {
                    true => None,
                    false => Some(Json(message.attachments.clone())),
                } as _,
                Json(message.author.clone()) as _,
                message.channel_id.get() as i64,
                match message.components.is_empty() {
                    true => None,
                    false => Some(Json(message.components.clone())),
                } as _,
                message.content,
                true, // Message Deleted
                message.edited_timestamp.map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
                match message.embeds.is_empty() {
                    true => None,
                    false => Some(Json(message.embeds.clone())),
                } as _,
                message.flags.map(|x|Json(x)) as _,
                message.guild_id.map(|x|x.get() as i64),
                message.id.get() as i64,
                message.interaction.clone().map(|x|Json(x)) as _,
                Json(message.kind) as _,
                match message.mention_channels.is_empty() {
                    true => None,
                    false => Some(Json(message.mention_channels.clone())),
                } as _,
                message.mention_everyone,
                match message.mention_roles.is_empty() {
                    true => None,
                    false => Some(message.mention_roles.clone().into_iter().map(|x|x.get() as i64).collect::<Vec<_>>()
                ),
                } as _,
                match message.mentions.is_empty() {
                    true => None,
                    false => Some(Json(message.mentions.clone())),
                } as _, 
                message.pinned,
                match message.reactions.is_empty() {
                    true => None,
                    false => Some(Json(message.reactions.clone())),
                } as _,
                message.reference.clone().map(|x|Json(x)) as _,
                message.referenced_message.clone().map(|x|Json(x)) as _,
                message.role_subscription_data.clone().map(|x|Json(x)) as _,
                DatabaseMessageSource::MessageCreate as _,
                match message.sticker_items.is_empty() {
                    true => None,
                    false => Some(Json(message.sticker_items.clone())),
                } as _,
                message.thread.clone().map(|x|Json(x)) as _,
                OffsetDateTime::from_unix_timestamp(message.timestamp.as_secs()).unwrap(),
                message.tts,
                message.webhook_id.map(|x|x.get() as i64),
                message.member.clone().map(Json) as _
            );

            query.fetch_optional(&self.0).await.map(|x| x.map(|x|x.into()))
        }

    async fn handle_message_delete(&self, message: MessageDelete) -> Result<Option<LuroMessage>, Error> {
        debug!("Handling message_delete {:#?}", message);

        let query = sqlx::query_as!(
            DatabaseMessage,
            "UPDATE messages
            SET
                channel_id = $1,
                guild_id = $2,
                id = $3,
                source = $4
            WHERE id = $3
            RETURNING
                activity as \"activity: Json<MessageActivity>\",
                application_id,
                application as \"application: Json<MessageApplication>\",
                attachments as \"attachments: Json<Vec<Attachment>>\",
                author as \"author: Json<User>\",
                channel_id,
                components as \"components: Json<Vec<Component>>\",
                content,
                deleted,
                edited_timestamp,
                embeds as \"embeds: Json<Vec<Embed>>\",
                flags as \"flags: Json<MessageFlags>\",
                guild_id,
                id,
                interaction as \"interaction: Json<MessageInteraction>\",
                kind as \"kind: Json<MessageType>\", 
                mention_channels as \"mention_channels: Json<Vec<ChannelMention>>\",
                mention_everyone,
                mention_roles as \"mention_roles: Vec<i64>\",
                mentions as \"mentions: Json<Vec<Mention>>\",
                pinned,
                reactions as \"reactions: Json<Vec<Reaction>>\",
                reference as \"reference: Json<MessageReference>\",
                referenced_message as \"referenced_message: Json<Box<Message>>\",
                role_subscription_data as \"role_subscription_data: Json<RoleSubscriptionData>\",
                source as \"source: DatabaseMessageSource\",
                sticker_items as \"sticker_items: Json<Vec<MessageSticker>>\",
                member as \"member: Json<PartialMember>\",

                thread as \"thread: Json<Channel>\",
                timestamp,
                tts,
                webhook_id,
                message_updates as \"message_updates: Json<Vec<MessageUpdate>>\"
            ",
            message.channel_id.get() as i64,
            message.guild_id.map(|x| x.get() as i64),
            message.id.get() as i64,
            DatabaseMessageSource::MessageDelete as _,
        );

        query.fetch_optional(&self.0).await.map(|x| x.map(|x|x.into()))
    }

    async fn handle_luro_message(&self, message: LuroMessage) -> Result<Option<LuroMessage>, Error> {
        let message: DatabaseMessage = message.into();

        debug!("Inserting {:#?}", message);

        let query = sqlx::query_as!(
            DatabaseMessage,
            "INSERT INTO messages (
                activity,
                application_id,
                application,
                attachments,
                author,
                channel_id,
                components,
                content,
                deleted,
                edited_timestamp,
                embeds,
                flags,
                guild_id,
                id,
                interaction,
                kind, 
                mention_channels,
                mention_everyone,
                mention_roles,
                mentions,
                pinned,
                reactions,
                reference,
                referenced_message,
                role_subscription_data,
                source,
                sticker_items,
                thread,
                timestamp,
                tts,
                webhook_id,
                member
            ) VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11 , $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32)
            ON CONFLICT (id)
            DO UPDATE SET
                activity = $1,
                application_id = $2,
                application = $3,
                attachments = $4,
                author = $5,
                channel_id = $6,
                components = $7,
                content = $8,
                deleted = $9,
                edited_timestamp = $10,
                embeds = $11,
                flags = $12,
                guild_id = $13,
                id = $14,
                interaction = $15,
                kind = $16,
                mention_channels = $17,
                mention_everyone = $18,
                mention_roles = $19,
                mentions = $20,
                pinned = $21,
                reactions = $22,
                reference = $23,
                referenced_message = $24,
                role_subscription_data = $25,
                source = $26,
                sticker_items = $27,
                thread = $28,
                timestamp = $29,
                tts = $30,
                webhook_id = $31,
                member = $32
            RETURNING
                activity as \"activity: Json<MessageActivity>\",
                application_id,
                application as \"application: Json<MessageApplication>\",
                attachments as \"attachments: Json<Vec<Attachment>>\",
                author as \"author: Json<User>\",
                channel_id,
                components as \"components: Json<Vec<Component>>\",
                content,
                deleted,
                edited_timestamp,
                embeds as \"embeds: Json<Vec<Embed>>\",
                flags as \"flags: Json<MessageFlags>\",
                guild_id,
                id,
                interaction as \"interaction: Json<MessageInteraction>\",
                kind as \"kind: Json<MessageType>\", 
                mention_channels as \"mention_channels: Json<Vec<ChannelMention>>\",
                mention_everyone,
                mention_roles as \"mention_roles: Vec<i64>\",
                mentions as \"mentions: Json<Vec<Mention>>\",
                pinned,
                reactions as \"reactions: Json<Vec<Reaction>>\",
                member as \"member: Json<PartialMember>\",

                reference as \"reference: Json<MessageReference>\",
                referenced_message as \"referenced_message: Json<Box<Message>>\",
                role_subscription_data as \"role_subscription_data: Json<RoleSubscriptionData>\",
                source as \"source: DatabaseMessageSource\",
                sticker_items as \"sticker_items: Json<Vec<MessageSticker>>\",
                thread as \"thread: Json<Channel>\",
                timestamp,
                tts,
                webhook_id,
                message_updates as \"message_updates: Json<Vec<MessageUpdate>>\"    
            ",
            message.activity as _,
            message.application_id,
            message.application as _,
            message.attachments as _,
            message.author as _,
            message.channel_id,
            message.components as _,
            message.content,
            message.deleted,
            message.edited_timestamp,
            message.embeds as _,
            message.flags as _,
            message.guild_id,
            message.id,
            message.interaction as _,
            message.kind as _,
            message.mention_channels as _,
            message.mention_everyone,
            message.mention_roles as _,
            message.mentions as _,
            message.pinned,
            message.reactions as _,
            message.reference as _,
            message.referenced_message as _,
            message.role_subscription_data as _,
            message.source as _,
            message.sticker_items as _,
            message.thread as _,
            message.timestamp,
            message.tts,
            message.webhook_id,
            message.member as _
        );

        query.fetch_optional(&self.0).await.map(|x| x.map(|x|x.into()))
    }
}
