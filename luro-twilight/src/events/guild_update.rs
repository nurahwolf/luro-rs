use luro_framework::LuroContext;
use twilight_model::gateway::payload::incoming::GuildUpdate;

pub async fn guild_update_listener(ctx: LuroContext, event: Box<GuildUpdate>) -> anyhow::Result<()> {
    ctx.database.guild_update(event).await?;

    Ok(())
}
