use luro_model::message::LuroMessage;
use sqlx::types::Json;
use sqlx::Error;
use twilight_model::channel::message::sticker::MessageSticker;
use twilight_model::channel::message::Mention;
use twilight_model::channel::Channel;
use twilight_model::channel::{
    message::{
        Component, Embed, MessageActivity, MessageApplication, MessageFlags, MessageInteraction, MessageReference, MessageType, Reaction,
        RoleSubscriptionData,
    },
    Attachment, ChannelMention, Message,
};
use twilight_model::gateway::payload::incoming::MessageUpdate;
use twilight_model::guild::PartialMember;
use twilight_model::user::User;

use crate::{DatabaseMessage, DatabaseMessageSource, LuroDatabase};

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
                member as \"member: Json<PartialMember>\",
                timestamp,
                tts,
                webhook_id,
                message_updates as \"message_updates: Json<Vec<MessageUpdate>>\"
            FROM messages WHERE message_id = $1",
            id
        );

        query.fetch_optional(&self.pool).await.map(|x| x.map(|x| x.into()))
    }
}
