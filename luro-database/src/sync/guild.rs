use twilight_model::gateway::payload::incoming::GuildUpdate;

pub async fn update(db: &crate::Database, event: &GuildUpdate) -> anyhow::Result<()> {
    db.guild_update(event).await?;

    Ok(())
}
