use twilight_model::gateway::payload::incoming::UserUpdate;

use crate::database::sqlx::{Database, Error};

pub async fn update(db: &Database, event: &UserUpdate) -> Result<(), Error> {
    tracing::info!("User {} updated", event.id);

    db.update_user(event).await?;

    Ok(())
}
