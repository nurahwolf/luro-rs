use twilight_model::gateway::payload::incoming::{GuildCreate, GuildUpdate};

pub async fn update(db: &crate::Database, event: &GuildUpdate) -> anyhow::Result<()> {
    db.guild_update(event).await?;

    Ok(())
}

pub async fn create(db: &crate::Database, event: &GuildCreate) -> anyhow::Result<()> {
    // Ensure a channel is present in the database first
    for channel in &event.channels {
        db.channel_update(channel.id).await?;
    }

    db.guild_update(event).await?;

    // Now that both guild and channels are present, update the channel data fully
    for channel in &event.channels {
        db.channel_update(channel).await?;
    }

    for role in &event.roles {
        db.role_update((event.id, role)).await?;
    }

    for member in &event.members {
        if let Err(why) = db.member_update((event.id, member)).await {
            tracing::warn!(why = ?why, "guild_create - Failed to sync member {}", member.user.id)
        }
    }

    Ok(())
}
