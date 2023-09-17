use anyhow::anyhow;
use luro_framework::{CommandInteraction, ComponentInteraction, Context, ModalInteraction};

use tracing::error;
use twilight_model::{
    application::interaction::{InteractionData, InteractionType},
    gateway::payload::incoming::InteractionCreate,
};

use crate::commands::{handle_autocomplete, handle_command, handle_component, handle_modal};

pub async fn interaction_create_listener(ctx: Context, event: Box<InteractionCreate>) -> anyhow::Result<()> {
    let response = match event.kind {
        InteractionType::Ping => Err(anyhow!("Got ping command, which has not handler: {:#?}", event)),
        InteractionType::ApplicationCommand => match event.data.clone() {
            Some(InteractionData::ApplicationCommand(data)) => {
                handle_command(CommandInteraction::new(ctx, event, data, ())).await
            }
            _ => Err(anyhow!("Got unknown interaction: {:#?}", event)),
        },
        InteractionType::MessageComponent => match event.data.clone() {
            Some(InteractionData::MessageComponent(data)) => {
                handle_component(ComponentInteraction::new(ctx, event, data, ())).await
            }
            _ => Err(anyhow!("Got unknown interaction: {:#?}", event)),
        },
        InteractionType::ApplicationCommandAutocomplete => match event.data.clone() {
            Some(InteractionData::ApplicationCommand(data)) => {
                handle_autocomplete(CommandInteraction::new(ctx, event, data, ())).await
            }
            _ => Err(anyhow!("Got unknown interaction: {:#?}", event)),
        },
        InteractionType::ModalSubmit => match event.data.clone() {
            Some(InteractionData::ModalSubmit(data)) => handle_modal(ModalInteraction::new(ctx, event, data, ())).await,
            _ => Err(anyhow!("Got unknown interaction: {:#?}", event)),
        },
        data => Err(anyhow!("Got unknown interaction: {:#?}", data)),
    };

    // TODO: Really shitty event handler, please change this
    if let Err(why) = response {
        error!(why = ?why, "error while handling event");
    }

    Ok(())
}
