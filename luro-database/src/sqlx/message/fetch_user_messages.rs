use std::collections::BTreeMap;

use futures_util::TryStreamExt;
use luro_model::message::LuroMessage;
use sqlx::types::Json;

use twilight_model::channel::message::sticker::MessageSticker;
use twilight_model::channel::message::Mention;
use twilight_model::channel::Channel;
use twilight_model::gateway::payload::incoming::MessageUpdate;
use twilight_model::guild::PartialMember;
use twilight_model::id::marker::UserMarker;
use twilight_model::user::User;
use twilight_model::{
    channel::{
        message::{
            Component, Embed, MessageActivity, MessageApplication, MessageFlags, MessageInteraction, MessageReference, MessageType,
            Reaction, RoleSubscriptionData,
        },
        Attachment, ChannelMention, Message,
    },
    id::Id,
};

use crate::{DatabaseMessage, DatabaseMessageSource, LuroDatabase};

impl LuroDatabase {
    pub async fn fetch_user_messages(&self, user_id: Id<UserMarker>) -> BTreeMap<i64, LuroMessage> {
        let mut messages = BTreeMap::new();
        let mut query = sqlx::query_as!(
            DatabaseMessage,
            "SELECT 
                activity as \"activity: Json<MessageActivity>\",
                application_id,
                application as \"application: Json<MessageApplication>\",
                attachments as \"attachments: Json<Vec<Attachment>>\",
                author as \"author: Json<User>\",
                author_id,
                channel_id,
                components as \"components: Json<Vec<Component>>\",
                content,
                deleted,
                edited_timestamp,
                embeds as \"embeds: Json<Vec<Embed>>\",
                flags as \"flags: Json<MessageFlags>\",
                guild_id,
                message_id,
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
            FROM messages
            WHERE author_id = $1
            ORDER BY message_id DESC
            LIMIT 25
        ",
        user_id.get() as i64
        )
        .fetch(&self.pool);

        while let Ok(Some(message)) = query.try_next().await {
            messages.insert(message.message_id, message.into());
        }

        tracing::info!("Returning {} messages!", messages.len());
        messages
    }
}
