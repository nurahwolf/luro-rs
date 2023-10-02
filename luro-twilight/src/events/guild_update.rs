use luro_framework::Context;
use tracing::debug;
use twilight_model::gateway::payload::incoming::GuildUpdate;

pub async fn guild_update_listener(ctx: Context, event: Box<GuildUpdate>) -> anyhow::Result<()> {
    debug!("Message Updated");

    ctx.database.update_guild(event).await?;

    Ok(())
}
