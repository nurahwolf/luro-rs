use luro_framework::Context;
use tracing::info;
use twilight_model::gateway::payload::incoming::RoleUpdate;

pub async fn role_update_listener(framework: Context, event: RoleUpdate) -> anyhow::Result<()> {
    info!("Role {} updated in guild {}", event.role.id, event.guild_id);

    let mut guild = framework.database.get_guild(&event.guild_id).await?;
    guild.roles.insert(event.role.id, event.role.into());
    framework.database.modify_guild(&event.guild_id, &guild).await?;

    Ok(())
}
