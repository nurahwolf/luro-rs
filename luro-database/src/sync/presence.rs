use twilight_model::gateway::payload::incoming::PresenceUpdate;

pub async fn update(db: &crate::Database, event: &PresenceUpdate) -> anyhow::Result<()> {
    tracing::debug!("presence_update - User {} updated", event.user.id());

    if let twilight_model::gateway::presence::UserOrId::User(user) = &event.user {
        if let Err(why) = db.user_update(user).await {
            tracing::warn!(why = ?why, "presence_update- Failed to sync user to database ")
        }
    }

    Ok(())
}
