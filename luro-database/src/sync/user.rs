use twilight_model::gateway::payload::incoming::UserUpdate;

use crate::Database;

pub async fn update(db: &Database, event: &UserUpdate) -> anyhow::Result<()> {
    tracing::info!("User {} updated", event.id);

    db.user_update(event).await?;

    Ok(())
}
