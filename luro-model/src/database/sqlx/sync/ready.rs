use twilight_model::gateway::payload::incoming::Ready;

use crate::database::sqlx::{Database, Error};

pub async fn ready(db: &Database, event: &Ready) -> Result<(), Error> {
    tracing::debug!("ready - Received ready event, flushing data to database");

    if let Err(why) = db.update_application(&event.application).await {
        tracing::warn!(why = ?why, "ready - Failed to sync application data to the database")
    }

    for guild in &event.guilds {
        if let Err(why) = db.update_guild(guild).await {
            tracing::warn!(why = ?why, "READY: Failed to sync guild {guild:?}")
        }
    }

    if let Err(why) = db.update_user(&event.user).await {
        tracing::warn!(why = ?why, "ready - Failed to sync current user data to the database")
    }

    Ok(())
}
