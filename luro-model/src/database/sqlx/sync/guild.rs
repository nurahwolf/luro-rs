use twilight_model::gateway::payload::incoming::{GuildCreate, GuildUpdate};

use crate::database::sqlx::{Database, Error};

pub async fn update(db: &Database, event: &GuildUpdate) -> Result<(), Error> {
    db.update_guild(event).await?;

    Ok(())
}

pub async fn create(db: &Database, event: &GuildCreate) -> Result<(), Error> {
    // Ensure a channel is present in the database first
    for channel in &event.channels {
        db.update_channel(channel.id).await?;
    }

    db.update_guild(event).await?;

    // Now that both guild and channels are present, update the channel data fully
    for channel in &event.channels {
        db.update_channel(channel).await?;
    }

    for role in &event.roles {
        db.update_role((event.id, role)).await?;
    }

    for member in &event.members {
        if let Err(why) = db.update_user((event.id, member)).await {
            tracing::warn!(why = ?why, "guild_create - Failed to sync member {}", member.user.id)
        }
    }

    Ok(())
}
