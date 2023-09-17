use anyhow::anyhow;
use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{application::interaction::application_command::CommandData, guild::Permissions};

use crate::responses::Response;
use crate::slash_command::LuroCommand;
use crate::{CommandInteraction, ComponentInteraction, ModalInteraction};

pub trait CreateLuroCommand: CommandModel {
    /// Create a new command and get it's data from the interaction
    fn new_command(data: Box<CommandData>) -> anyhow::Result<Self>
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

/// Add some custom functionality around [CommandModel]
#[async_trait]
pub trait LuroCommandTrait: CommandModel + CreateCommand + Send + Sync + 'static {
    fn new_command() -> LuroCommand<Self> {
        LuroCommand {
            name: Self::NAME,
            create: Self::create_command,
            interaction_command: Self::handle_interaction,
            component: Self::handle_component,
            modal: Self::handle_modal,
            autocomplete: Self::handle_autocomplete,
        }
    }

    /// Create a new command and get it's data from the interaction
    fn new(data: Box<CommandData>) -> anyhow::Result<Self> {
        let data = *data;
        match Self::from_interaction(data.into()) {
            Ok(ok) => Ok(ok),
            Err(why) => Err(anyhow!(
                "Got interaction data, but failed to parse it to the command type specified: {why}"
            )),
        }
    }

    /// Run the command / command group
    async fn handle_interaction(ctx: CommandInteraction<Self>) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Handle a component interaction. This could be a button or other form of interaciton
    async fn handle_component(ctx: ComponentInteraction<Self>) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Create and respond to a button interaction
    async fn handle_modal(ctx: ModalInteraction<Self>) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Create and respond to a button interaction
    async fn handle_autocomplete(ctx: CommandInteraction<Self>) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// The default permissions a user needs to run this command
    fn default_permissions() -> Permissions {
        Permissions::all()
    }
}
