use luro_model::message::LuroMessage;
use sqlx::types::Json;
use sqlx::Error;
use tracing::debug;
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
    pub async fn handle_luro_message(&self, message: LuroMessage) -> Result<Option<LuroMessage>, Error> {
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
                message_id,
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
            ON CONFLICT (message_id)
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
            author_id,
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
            message.message_id,
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

        query.fetch_optional(&self.pool).await.map(|x| x.map(|x| x.into()))
    }
}
