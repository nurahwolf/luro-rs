use luro_framework::{Context, Luro};
use tracing::info;
use twilight_model::gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate};

pub async fn role_update_listener(ctx: Context, event: RoleUpdate) -> anyhow::Result<()> {
    info!("Role {} updated in guild {}", event.role.id, event.guild_id);

    let mut guild = ctx.get_guild(&event.guild_id).await?;
    guild.roles.insert(event.role.id, event.role.clone().into());
    ctx.database.update_guild(guild).await?;
    ctx.database.update_role(event.role).await?;

    Ok(())
}

pub async fn role_create_listener(ctx: Context, event: RoleCreate) -> anyhow::Result<()> {
    info!("Role {} created in guild {}", event.role.id, event.guild_id);

    let mut guild = ctx.get_guild(&event.guild_id).await?;
    guild.roles.insert(event.role.id, event.role.clone().into());
    ctx.database.update_guild(guild).await?;
    ctx.database.update_role(event.role).await?;

    Ok(())
}

pub async fn role_delete_listener(ctx: Context, event: RoleDelete) -> anyhow::Result<()> {
    info!("Role {} deleted in guild {}", event.role_id, event.guild_id);

    ctx.database.delete_role(event.role_id).await?;

    Ok(())
}
