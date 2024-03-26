use twilight_model::gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate};

use crate::database::sqlx::{Database, Error};

pub async fn update(db: &Database, event: &MemberUpdate) -> Result<(), Error> {
    tracing::debug!("member_update - Member {} in guild {}", event.user.id, event.guild_id);

    db.update_user(event).await?;

    Ok(())
}

pub async fn add(db: &Database, event: &MemberAdd) -> Result<(), Error> {
    tracing::debug!("member_add - Member {} in guild {}", event.user.id, event.guild_id);

    db.update_user(event).await?;

    Ok(())
}

pub async fn delete(db: &Database, event: &MemberRemove) -> Result<(), Error> {
    tracing::debug!("member_remove - Member {} in guild {}", event.user.id, event.guild_id);

    db.update_user(event).await?;

    Ok(())
}

pub async fn chunk(db: &Database, event: &MemberChunk) -> Result<(), Error> {
    tracing::debug!("member_chunk - In guild {}", event.guild_id);

    db.update_user(event).await?;

    Ok(())
}
