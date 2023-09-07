use std::sync::Arc;

use luro_framework::{Framework, InteractionCommand, InteractionComponent, InteractionContext, InteractionModal};
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::{error, warn};
use twilight_model::application::interaction::{InteractionData, InteractionType};

use crate::commands::{handle_autocomplete, handle_command, handle_component, handle_modal};

pub async fn interaction_create_listener<D: LuroDatabaseDriver>(
    framework: Arc<Framework<D>>,
    interaction: InteractionContext
) -> anyhow::Result<()> {
    let data = match interaction.data.clone() {
        Some(data) => data,
        None => {
            warn!(interaction = ?interaction, "Interaction without any data!");
            return Ok(());
        }
    };

    let response = match data {
        InteractionData::ApplicationCommand(data) => match &interaction.kind {
            InteractionType::ApplicationCommand => handle_command(framework, InteractionCommand::new(interaction, data)).await,
            InteractionType::ApplicationCommandAutocomplete => {
                handle_autocomplete(framework, InteractionCommand::new(interaction, data)).await
            }
            _ => {
                warn!(interaction = ?interaction, "Application Command with unexpected application data!");
                Ok(())
            }
        },
        InteractionData::MessageComponent(data) => {
            handle_component(framework, InteractionComponent::new(interaction, data)?).await
        }
        InteractionData::ModalSubmit(data) => handle_modal(framework, InteractionModal::new(interaction, data)).await,
        _ => todo!()
    };

    // TODO: Really shitty event handler, please change this
    if let Err(why) = response {
        error!(why = ?why, "error while handling event");
    }

    Ok(())
}
