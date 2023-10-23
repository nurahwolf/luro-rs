use luro_model::message::LuroMessage;
use sqlx::types::Json;
use time::OffsetDateTime;
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
    pub async fn handle_message(&self, message: Message) -> anyhow::Result<Option<LuroMessage>> {
        debug!("Handling message_create {:#?}", message);

        self.update_user(message.author.clone()).await?;

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
                    ($1, $2, $3, $4, $5, $6, $7, $8, false, $9, $10, $11 , $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31)
                ON CONFLICT
                    (message_id)
                DO UPDATE SET
                    activity = $1,
                    application_id = $2,
                    application = $3,
                    attachments = $4,
                    author = $5,
                    channel_id = $6,
                    components = $7,
                    content = $8,
                    deleted = false,
                    edited_timestamp = $9,
                    embeds = $10,
                    flags = $11,
                    guild_id = $12,
                    interaction = $14,
                    kind = $15,
                    mention_channels = $16,
                    mention_everyone = $17,
                    mention_roles = $18,
                    mentions = $19,
                    pinned = $20,
                    reactions = $21,
                    reference = $22,
                    referenced_message = $23,
                    role_subscription_data = $24,
                    source = $25,
                    sticker_items = $26,
                    thread = $27,
                    timestamp = $28,
                    tts = $29,
                    webhook_id = $30,
                    member = $31
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

        Ok(query.fetch_optional(&self.pool).await.map(|x| x.map(|x| x.into()))?)
    }
}
