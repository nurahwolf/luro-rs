use luro_framework::{Context, Luro};
use tracing::info;
use twilight_model::gateway::payload::incoming::RoleUpdate;

pub async fn role_update_listener(framework: Context, event: RoleUpdate) -> anyhow::Result<()> {
    info!("Role {} updated in guild {}", event.role.id, event.guild_id);

    let mut guild = framework.get_guild(&event.guild_id).await?;
    guild.roles.insert(event.role.id, event.role.into());
    framework.database.update_guild(guild).await?;

    Ok(())
}
