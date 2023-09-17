use anyhow::anyhow;
use async_trait::async_trait;
use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::application_command::CommandData;

use crate::responses::Response;
use crate::{CommandInteraction, ComponentInteraction, ModalInteraction};

pub trait CreateLuroCommand: CommandModel {
    /// Create a new command and get it's data from the interaction
    fn new(data: Box<CommandData>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let data = *data;
        match Self::from_interaction(data.into()) {
            Ok(ok) => Ok(ok),
            Err(why) => Err(anyhow!(
                "Got interaction data, but failed to parse it to the command type specified: {why}"
            )),
        }
    }
}

#[async_trait]
pub trait ExecuteLuroCommand: Send + Sync {
    /// Run the command / command group
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Handle a component interaction. This could be a button or other form of interaciton
    async fn interaction_component(&self, ctx: ComponentInteraction<()>) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Create and respond to a button interaction
    async fn interaction_modal(&self, ctx: ModalInteraction<()>) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Create and respond to a button interaction
    async fn interaction_autocomplete(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }
}
