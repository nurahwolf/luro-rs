use twilight_model::gateway::payload::incoming::{GuildUpdate, GuildCreate};

pub async fn update(db: &crate::Database, event: &GuildUpdate) -> anyhow::Result<()> {
    db.guild_update(event).await?;

    Ok(())
}

pub async fn create(db: &crate::Database, event: &GuildCreate) -> anyhow::Result<()> {
    db.guild_update(event).await?;

    Ok(())
}