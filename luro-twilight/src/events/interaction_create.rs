use luro_framework::{InteractionContext, LuroContext};

use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::commands::handle_interaction;

pub async fn interaction_create_listener(ctx: LuroContext, event: Box<InteractionCreate>) -> anyhow::Result<()> {
    if let Err(why) = handle_interaction(InteractionContext::new(ctx, event.0).await?).await {
        tracing::error!(why = ?why, "UNHANDLED EXCEPTION, PLEASE CREATE A HANDLER!");
    }

    Ok(())
}
