use luro_framework::{InteractionContext, LuroContext};

use tracing::{error, warn};
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::commands::handle_interaction;

pub async fn interaction_create_listener(ctx: LuroContext, event: Box<InteractionCreate>) -> anyhow::Result<()> {
    if let Some(channel) = &event.channel {
        if let Err(why) = ctx.database.update_channel(channel.clone()).await {
            warn!("interaction_handler - Failed to update channel: {why}")
        }
    }

    if let Some(user) = &event.user {
        if let Err(why) = ctx.database.update_user(user.clone()).await {
            warn!("interaction_handler - Failed to update user: {why}")
        }
    }

    if let Some(guild_id) = event.guild_id && let Some(member) = &event.member {
        if let Err(why) = ctx.database.update_member((guild_id, member.clone())).await {
            warn!("interaction_handler - Failed to update partial member: {why}")
        }
    }

    if let Err(why) = ctx.database.update_interaction(event.0.clone().into()).await {
        warn!("interaction_handler - Failed to update interaction: {why}")
    }

    if let Err(why) = handle_interaction(InteractionContext::new(ctx, event.0).await?).await {
        error!(why = ?why, "UNHANDLED EXCEPTION, PLEASE CREATE A HANDLER!");
    }

    Ok(())
}
