use luro_model::sync::MessageSync;
use twilight_model::gateway::payload::incoming::{MessageDeleteBulk, MessageCreate, MessageDelete, MessageUpdate};

use crate::Database;

pub async fn create(db: &Database, event: &MessageCreate) -> anyhow::Result<()> {
    tracing::debug!("Message Received");

    db.message_update(MessageSync::MessageCreate(event)).await?;

    Ok(())
}

pub async fn delete_bulk(db: &Database, event: &MessageDeleteBulk) -> anyhow::Result<()> {
    tracing::debug!("Messages Bulk Deleted");

    db.message_update(MessageSync::MessageDeleteBulk(event)).await?;

    Ok(())
}

pub async fn delete(db: &Database, event: &MessageDelete) -> anyhow::Result<()> {
    tracing::debug!("Message Deleted");

    db.message_update(MessageSync::MessageDelete(event)).await?;

    Ok(())
}

pub async fn update(db: &Database, event: &MessageUpdate) -> anyhow::Result<()> {
    tracing::debug!("Message Updated");

    db.message_update(MessageSync::MessageUpdate(event)).await?;

    Ok(())
}