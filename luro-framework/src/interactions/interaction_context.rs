use anyhow::anyhow;
use luro_model::response::LuroResponse;
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    http::interaction::InteractionResponseType,
};

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

    pub async fn respond<F>(&self, response: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse,
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        let (interaction_id, interaction_token, interaction_client) = match self {
            InteractionContext::CommandInteraction(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::CommandAutocompleteInteraction(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::ComponentInteraction(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::ModalInteraction(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
        };

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage {
            true => interaction_client.update_response(interaction_token).embeds(r.embeds.as_deref()).await?.status(),
            false => interaction_client.create_response(interaction_id, interaction_token, &r.interaction_response()).await?.status(),
        };

        Ok(())
    }

    pub async fn simple_response(&self, response: Response<'_>) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
