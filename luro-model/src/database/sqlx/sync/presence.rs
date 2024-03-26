use twilight_model::gateway::payload::incoming::PresenceUpdate;

use crate::database::sqlx::{Database, Error};

pub async fn update(db: &Database, event: &PresenceUpdate) -> Result<(), Error> {
    tracing::debug!("presence_update - User {} updated", event.user.id());

    if let twilight_model::gateway::presence::UserOrId::User(user) = &event.user {
        if let Err(why) = db.update_user(user).await {
            tracing::warn!(why = ?why, "presence_update - Failed to sync user to database ")
        }
    }

    Ok(())
}
