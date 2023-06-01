use anyhow::Error;
use std::sync::Arc;
use tracing::info;
use twilight_gateway::MessageSender;
use twilight_model::{
    application::interaction::InteractionData, gateway::payload::incoming::InteractionCreate,
};

use crate::{commands, models::luro::Luro};

pub async fn interaction_create_listener(
    luro: Arc<Luro>,
    interaction: Box<InteractionCreate>,
    _shard: MessageSender,
) -> Result<(), Error> {
    info!("Interaction received!");

    match &interaction.data {
        Some(InteractionData::ApplicationCommand(command)) => {
            if let Some(channel) = &interaction.channel && let Some(user) = &interaction.user {
                    tracing::info!(
                        "{} command in channel {} by {}",
                        command.name,
                        channel.id,
                        user.name
                    );
                };

            // commands::handle_command(&luro, &interaction, command, shard).await;
        }
        Some(InteractionData::MessageComponent(component)) => {
            commands::handle_component(&luro, &interaction, component).await;
        }
        _ => {}
    };

    Ok(())
}
