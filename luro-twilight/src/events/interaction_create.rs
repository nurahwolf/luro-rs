use luro_framework::{Context, InteractionContext};

use tracing::{error, warn};
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::commands::handle_interaction;

pub async fn interaction_create_listener(ctx: Context, event: Box<InteractionCreate>) -> anyhow::Result<()> {
    if let Some(channel) = &event.channel {
        if let Err(why) = ctx.database.update_channel(channel.clone()).await {
            warn!("Failed to update channel: {why}")
        }
    }

    if let Some(guild_id) = event.guild_id && let Some(member) = &event.member {
        if let Err(why) = ctx.database.update_member((guild_id, member.clone())).await {
            warn!("Failed to update partial member: {why}")
        }
    }

    if let Err(why) = ctx.database.update_interaction(event.0.clone().into()).await {
        warn!("Failed to update interaction: {why}")
    }

    // TODO: Really shitty event handler, please change this
    if let Err(why) = handle_interaction(InteractionContext::new(ctx, event.0)?).await {
        error!(why = ?why, "error while handling event");
    }

    Ok(())
}
