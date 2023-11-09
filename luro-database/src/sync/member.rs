use twilight_model::gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate};

pub async fn update(db: &crate::Database, event: &MemberUpdate) -> anyhow::Result<()> {
    tracing::debug!("member_update - Member {} in guild {}", event.user.id, event.guild_id);

    db.member_update(event).await?;

    Ok(())
}

pub async fn add(db: &crate::Database, event: &MemberAdd) -> anyhow::Result<()> {
    tracing::debug!("member_add - Member {} in guild {}", event.user.id, event.guild_id);

    db.member_update(event).await?;

    Ok(())
}

pub async fn delete(db: &crate::Database, event: &MemberRemove) -> anyhow::Result<()> {
    tracing::debug!("member_remove - Member {} in guild {}", event.user.id, event.guild_id);

    db.member_update(event).await?;

    Ok(())
}

pub async fn chunk(db: &crate::Database, event: &MemberChunk) -> anyhow::Result<()> {
    tracing::debug!("member_chunk - In guild {}", event.guild_id);


    db.member_update(event).await?;

    Ok(())
}
