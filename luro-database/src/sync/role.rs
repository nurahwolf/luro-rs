use twilight_model::gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate};

pub async fn role_update_listener(db: &crate::Database, event: &RoleUpdate) -> anyhow::Result<()> {
    tracing::debug!("role_update - Role {} in guild {}", event.role.id, event.guild_id);

    db.role_update(event.clone()).await?;

    Ok(())
}

pub async fn role_create_listener(db: &crate::Database, event: &RoleCreate) -> anyhow::Result<()> {
    tracing::debug!("role_create - Role {} in guild {}", event.role.id, event.guild_id);

    db.role_update(event.clone()).await?;

    Ok(())
}

pub async fn role_delete_listener(db: &crate::Database, event: &RoleDelete) -> anyhow::Result<()> {
    tracing::debug!("role_delete - Role {} in guild {}", event.role_id, event.guild_id);

    db.role_update(event.clone()).await?;

    Ok(())
}
