use twilight_model::gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate};

pub async fn create(db: &crate::Database, event: &ChannelCreate) -> anyhow::Result<()> {
    tracing::debug!("channel_create - Channel {}", event.id);

    db.channel_update(event).await?;

    Ok(())
}

pub async fn pins_update(db: &crate::Database, event: &ChannelPinsUpdate) -> anyhow::Result<()> {
    tracing::debug!("channel_pins_update - Channel {}", event.channel_id);

    db.channel_update(event).await?;

    Ok(())
}

pub async fn delete(db: &crate::Database, event: &ChannelDelete) -> anyhow::Result<()> {
    tracing::debug!("channel_delete - Channel {}", event.id);

    db.channel_update(event).await?;

    Ok(())
}

pub async fn update(db: &crate::Database, event: &ChannelUpdate) -> anyhow::Result<()> {
    tracing::debug!("channel_update - Channel {}", event.id);

    db.channel_update(event).await?;

    Ok(())
}
