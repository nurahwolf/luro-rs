use twilight_model::gateway::payload::incoming::Ready;

pub async fn ready(db: &crate::Database, event: &Ready) -> anyhow::Result<()> {
    tracing::debug!("ready - Received ready event, flushing data to database");

    if let Err(why) = db.application_update(&event.application).await {
        tracing::warn!(why = ?why, "ready - Failed to sync application data to the database")
    }

    if let Err(why) = db.user_update(&event.user).await {
        tracing::warn!(why = ?why, "ready - Failed to sync current user data to the database")
    }

    Ok(())
}