use twilight_model::gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate};

use crate::database::sqlx::{Database, Error};

pub async fn create(db: &Database, event: &ChannelCreate) -> Result<(), Error> {
    tracing::debug!("channel_create - Channel {}", event.id);

    db.update_channel(event).await?;

    Ok(())
}

pub async fn pins_update(db: &Database, event: &ChannelPinsUpdate) -> Result<(), Error> {
    tracing::debug!("channel_pins_update - Channel {}", event.channel_id);

    db.update_channel(event).await?;

    Ok(())
}

pub async fn delete(db: &Database, event: &ChannelDelete) -> Result<(), Error> {
    tracing::debug!("channel_delete - Channel {}", event.id);

    db.update_channel(event).await?;

    Ok(())
}

pub async fn update(db: &Database, event: &ChannelUpdate) -> Result<(), Error> {
    tracing::debug!("channel_update - Channel {}", event.id);

    db.update_channel(event).await?;

    Ok(())
}
