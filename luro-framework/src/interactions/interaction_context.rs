use anyhow::anyhow;
use luro_model::response::LuroResponse;
use tracing::warn;
use twilight_model::application::interaction::{Interaction, InteractionType};

use crate::{responses::Response, CommandInteraction, ComponentInteraction, Context, InteractionContext, ModalInteraction};

impl InteractionContext {
    pub fn new(ctx: Context, interaction: Interaction) -> anyhow::Result<Self> {
        match interaction.kind {
            InteractionType::Ping => Err(anyhow!("Received ping interaction with no handler")),
            InteractionType::ApplicationCommand => Ok(Self::CommandInteraction(CommandInteraction::new(ctx, interaction)?)),
            InteractionType::MessageComponent => Ok(Self::ComponentInteraction(ComponentInteraction::new(ctx, interaction)?)),
            InteractionType::ApplicationCommandAutocomplete => {
                Ok(Self::CommandAutocompleteInteraction(CommandInteraction::new(ctx, interaction)?))
            }
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

    /// Create a simple response, genearlly used for interactions that don't exist and such
    pub async fn no_handler_response(&self, name: &str) -> anyhow::Result<()> {
        let no_handler_response = Response::UnknownCommand(name).embed();
        let mut luro_response = LuroResponse::default();
        let (interaction_id, interaction_token, interaction_client) = match self {
            InteractionContext::CommandInteraction(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::CommandAutocompleteInteraction(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::ComponentInteraction(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::ModalInteraction(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
        };

        luro_response.add_embed(no_handler_response.clone());
        let response = interaction_client
            .create_response(interaction_id, interaction_token, &luro_response.interaction_response())
            .await;

        if let Err(why) = response {
            warn!("Could not send new response, trying to update: {why}");
            interaction_client
                .update_response(interaction_token)
                .embeds(Some(&[no_handler_response.0]))
                .await?;
        }

        Ok(())
    }
}
