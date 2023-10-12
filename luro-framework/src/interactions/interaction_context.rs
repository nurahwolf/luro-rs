use anyhow::anyhow;
use twilight_model::application::interaction::{Interaction, InteractionType};

use crate::{CommandInteraction, ComponentInteraction, Context, InteractionContext, ModalInteraction};

impl InteractionContext {
    pub fn new(ctx: Context, interaction: Interaction) -> anyhow::Result<Self> {
        match interaction.kind {
            InteractionType::Ping => Err(anyhow!("Received ping interaction with no handler")),
            InteractionType::ApplicationCommand => Ok(Self::CommandInteraction(CommandInteraction::new(ctx, interaction)?)),
            InteractionType::MessageComponent => Ok(Self::ComponentInteraction(ComponentInteraction::new(ctx, interaction)?)),
            InteractionType::ApplicationCommandAutocomplete => Ok(Self::CommandAutocompleteInteraction(
                CommandInteraction::new(ctx, interaction)?,
            )),
            InteractionType::ModalSubmit => Ok(Self::ModalInteraction(ModalInteraction::new(ctx, interaction)?)),
            _ => Err(anyhow!("Unexpected interaction kind")),
        }
    }

    pub fn command_name(&self) -> &str {
        match self {
            Self::CommandAutocompleteInteraction(ctx) => ctx.command_name(),
            Self::CommandInteraction(ctx) => ctx.command_name(),
            Self::ComponentInteraction(ctx) => ctx.command_name(),
            Self::ModalInteraction(ctx) => ctx.command_name(),
        }
    }

    pub fn command_type(&self) -> &str {
        match self {
            Self::CommandAutocompleteInteraction(_) => "CommandAutocompleteInteraction",
            Self::CommandInteraction(_) => "CommandInteraction",
            Self::ComponentInteraction(_) => "ComponentInteraction",
            Self::ModalInteraction(_) => "ModalInteraction",
        }
    }
}
