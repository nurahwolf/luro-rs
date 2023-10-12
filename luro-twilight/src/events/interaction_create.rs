use luro_framework::{Context, InteractionContext};

use tracing::error;
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::commands::handle_interaction;

pub async fn interaction_create_listener(ctx: Context, event: Box<InteractionCreate>) -> anyhow::Result<()> {
    ctx.database.update_interaction(event.0.clone().into()).await?;

    // TODO: Really shitty event handler, please change this
    if let Err(why) = handle_interaction(InteractionContext::new(ctx, event.0)?).await {
        error!(why = ?why, "error while handling event");
    }

    Ok(())
}
