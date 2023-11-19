use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::Database;

pub async fn create(db: &Database, event: &InteractionCreate) -> anyhow::Result<()> {
    if let Some(channel) = &event.channel {
        if let Err(why) = db.channel_update(channel).await {
            tracing::warn!("interaction_handler - Failed to update channel: {why}")
        }
    }

    if let Some(user) = &event.user {
        if let Err(why) = db.user_update(user).await {
            tracing::warn!("interaction_handler - Failed to update user: {why}")
        }
    }

    if let Some(guild_id) = event.guild_id
        && let Some(member) = &event.member
    {
        if let Err(why) = db.member_update((guild_id, member)).await {
            tracing::warn!("interaction_handler - Failed to update partial member: {why}")
        }
    }

    if let Err(why) = db.interaction_update(&event.0).await {
        tracing::warn!("interaction_handler - Failed to update interaction: {why}")
    }

    Ok(())
}
