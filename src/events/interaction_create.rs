use tracing::info;
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::State;

pub async fn interaction_create_handler(
    _state: State,
    _interaction: Box<InteractionCreate>,
) -> anyhow::Result<()> {
    info!("Interaction created!");

    Ok(())
}
