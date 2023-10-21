use anyhow::anyhow;
use luro_model::response::LuroResponse;
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    http::interaction::InteractionResponseType,
};

use crate::{responses::Response, CommandInteraction, ComponentInteraction, LuroContext, ModalInteraction};

#[derive(Clone)]
pub enum InteractionContext {
    Command(CommandInteraction),
    CommandAutocomplete(CommandInteraction),
    Component(ComponentInteraction),
    Modal(ModalInteraction),
}

impl InteractionContext {
    pub fn new(ctx: LuroContext, interaction: Interaction) -> anyhow::Result<Self> {
        match interaction.kind {
            InteractionType::Ping => Err(anyhow!("Received ping interaction with no handler")),
            InteractionType::ApplicationCommand => Ok(Self::Command(CommandInteraction::new(ctx, interaction)?)),
            InteractionType::MessageComponent => Ok(Self::Component(ComponentInteraction::new(ctx, interaction)?)),
            InteractionType::ApplicationCommandAutocomplete => Ok(Self::CommandAutocomplete(CommandInteraction::new(ctx, interaction)?)),
            InteractionType::ModalSubmit => Ok(Self::Modal(ModalInteraction::new(ctx, interaction)?)),
            _ => Err(anyhow!("Unexpected interaction kind")),
        }
    }

    pub fn command_name(&self) -> &str {
        match self {
            Self::CommandAutocomplete(ctx) => ctx.command_name(),
            Self::Command(ctx) => ctx.command_name(),
            Self::Component(ctx) => ctx.command_name(),
            Self::Modal(ctx) => ctx.command_name(),
        }
    }

    pub fn command_type(&self) -> &str {
        match self {
            Self::CommandAutocomplete(_) => "CommandAutocompleteInteraction",
            Self::Command(_) => "CommandInteraction",
            Self::Component(_) => "ComponentInteraction",
            Self::Modal(_) => "ModalInteraction",
        }
    }

    pub async fn respond<F>(&self, response: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse,
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        let (interaction_id, interaction_token, interaction_client) = match self {
            InteractionContext::Command(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::CommandAutocomplete(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::Component(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
            InteractionContext::Modal(ctx) => (ctx.id, &ctx.token, ctx.interaction_client()),
        };

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            true => interaction_client
                .update_response(interaction_token)
                .embeds(r.embeds.as_deref())
                .await?
                .status(),
            false => interaction_client
                .create_response(interaction_id, interaction_token, &r.interaction_response())
                .await?
                .status(),
        };

        Ok(())
    }

    pub async fn simple_response(&self, response: Response<'_>) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
