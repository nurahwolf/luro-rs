use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::database::sqlx::{Database, Error};

pub async fn create(db: &Database, event: &InteractionCreate) -> Result<(), Error> {
    if let Some(channel) = &event.channel {
        if let Err(why) = db.update_channel(channel).await {
            tracing::warn!("interaction_handler - Failed to update channel: {why}")
        }
    }

    if let Some(user) = &event.user {
        if let Err(why) = db.update_user(user).await {
            tracing::warn!("interaction_handler - Failed to update user: {why}")
        }
    }

    if let Some(guild_id) = event.guild_id {
        if let Some(member) = &event.member {
            if let Err(why) = db.update_user((guild_id, member)).await {
                tracing::warn!("interaction_handler - Failed to update partial member: {why}")
            }
        }
    }

    if let Err(why) = db.update_interaction(&event.0).await {
        tracing::warn!("interaction_handler - Failed to update interaction: {why}")
    }

    Ok(())
}
