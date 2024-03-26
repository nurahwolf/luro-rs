use sqlx::{postgres::PgQueryResult, types::Json};
use time::OffsetDateTime;
use twilight_model::gateway::payload::incoming::{MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate};

use crate::{
    database::sqlx::{Database, Error},
    sync::MessageSync,
};

impl Database {
    pub async fn update_message(&self, message: impl Into<MessageSync<'_>>) -> Result<u64, Error> {
        match message.into() {
            MessageSync::Custom(message) => handle_luro_message(self, message).await,
            MessageSync::Message(message) => handle_twilight_message(self, message).await,
            MessageSync::MessageCreate(message) => handle_message_create(self, message).await,
            MessageSync::MessageDelete(message) => handle_message_delete(self, message).await,
            MessageSync::MessageDeleteBulk(messages) => handle_message_delete_bulk(self, messages).await,
            MessageSync::MessageUpdate(message) => handle_message_update(self, message).await,
        }
    }
}

async fn handle_message_create(db: &Database, message: &MessageCreate) -> Result<u64, Error> {
    tracing::debug!("Handling message_create {:#?}", message);
    let mut rows_updated = db.update_user(&message.author).await?;

    rows_updated += sqlx::query_file!(
        "queries/message_update_create.sql",
        message.activity.clone().map(|x| Json(x)) as _,
        message.application_id.map(|x| x.get() as i64),
        message.application.clone().map(|x| Json(x)) as _,
        Json(message.attachments.clone()) as _,
        Json(message.author.clone()) as _,
        message.channel_id.get() as i64,
        match message.components.is_empty() {
            true => None,
            false => Some(Json(message.components.clone())),
        } as _,
        message.content,
        match message.edited_timestamp {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        match message.embeds.is_empty() {
            true => None,
            false => Some(Json(message.embeds.clone())),
        } as _,
        message.flags.map(|x| Json(x)) as _,
        message.guild_id.map(|x| x.get() as i64),
        message.id.get() as i64,
        message.interaction.clone().map(|x| Json(x)) as _,
        Json(message.kind) as _,
        match message.mention_channels.is_empty() {
            true => None,
            false => Some(Json(message.mention_channels.clone())),
        } as _,
        message.mention_everyone,
        match message.mention_roles.is_empty() {
            true => None,
            false => Some(
                message
                    .mention_roles
                    .clone()
                    .into_iter()
                    .map(|x| x.get() as i64)
                    .collect::<Vec<_>>()
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
        message.reference.clone().map(|x| Json(x)) as _,
        message.referenced_message.clone().map(|x| Json(x)) as _,
        message.role_subscription_data.clone().map(|x| Json(x)) as _,
        DbMessageSource::MessageCreate as _,
        match message.sticker_items.is_empty() {
            true => None,
            false => Some(Json(message.sticker_items.clone())),
        } as _,
        message.thread.clone().map(|x| Json(x)) as _,
        OffsetDateTime::from_unix_timestamp(message.timestamp.as_secs())?,
        message.tts,
        message.webhook_id.map(|x| x.get() as i64),
        message.member.clone().map(Json) as _,
        message.author.id.get() as i64,
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())?;

    Ok(rows_updated)
}

pub async fn handle_message_update(db: &Database, message: &MessageUpdate) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query_file!(
        "queries/message_update_twilight_update.sql",
        message.id.get() as i64,
        Json(message) as _,
        DbMessageSource::MessageUpdate as _,
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())?)
}

async fn handle_twilight_message(db: &Database, message: &Message) -> anyhow::Result<u64> {
    let mut rows_updated = db.update_user(&message.author).await?;
    rows_updated += sqlx::query_file!(
        "queries/message_update_twilight_message.sql",
        message.activity.clone().map(|x| Json(x)) as _,
        message.application_id.map(|x| x.get() as i64),
        message.application.clone().map(|x| Json(x)) as _,
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
        message
            .edited_timestamp
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        match message.embeds.is_empty() {
            true => None,
            false => Some(Json(message.embeds.clone())),
        } as _,
        message.flags.map(|x| Json(x)) as _,
        message.guild_id.map(|x| x.get() as i64),
        message.id.get() as i64,
        message.interaction.clone().map(|x| Json(x)) as _,
        Json(message.kind) as _,
        match message.mention_channels.is_empty() {
            true => None,
            false => Some(Json(message.mention_channels.clone())),
        } as _,
        message.mention_everyone,
        match message.mention_roles.is_empty() {
            true => None,
            false => Some(
                message
                    .mention_roles
                    .clone()
                    .into_iter()
                    .map(|x| x.get() as i64)
                    .collect::<Vec<_>>()
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
        message.reference.clone().map(|x| Json(x)) as _,
        message.referenced_message.clone().map(|x| Json(x)) as _,
        message.role_subscription_data.clone().map(|x| Json(x)) as _,
        DbMessageSource::MessageCreate as _,
        match message.sticker_items.is_empty() {
            true => None,
            false => Some(Json(message.sticker_items.clone())),
        } as _,
        message.thread.clone().map(|x| Json(x)) as _,
        OffsetDateTime::from_unix_timestamp(message.timestamp.as_secs()).unwrap(),
        message.tts,
        message.webhook_id.map(|x| x.get() as i64),
        message.member.clone().map(Json) as _,
        message.author.id.get() as i64,
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    Ok(rows_updated)
}

async fn message_delete_bulk(db: &Database, messages: &MessageDeleteBulk) -> Result<u64, sqlx::Error> {
    let mut rows_updated = 0;
    for message in &messages.ids {
        rows_updated += message_delete(
            db,
            &MessageDelete {
                channel_id: messages.channel_id,
                guild_id: messages.guild_id,
                id: *message,
            },
        )
        .await?
    }

    Ok(rows_updated)
}

async fn message_delete(db: &Database, message: &MessageDelete) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_as!(
        DatabaseMessage,
        "
            UPDATE messages
            SET
                channel_id = $1,
                guild_id = $2,
                message_id = $3,
                source = $4,
                deleted = true
            WHERE message_id = $3
        ",
        message.channel_id.get() as i64,
        message.guild_id.map(|x| x.get() as i64),
        message.id.get() as i64,
        DbMessageSource::MessageDelete as _,
    )
    .execute(&db.pool)
    .await
}
