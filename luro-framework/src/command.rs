use anyhow::anyhow;
use tracing::warn;
use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::InteractionData;

use crate::responses::Response;
use crate::{CommandInteraction, ComponentInteraction, ModalInteraction};

pub trait CreateLuroCommand: CommandModel + ExecuteLuroCommand {
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

    /// Execute the command / command group
    async fn run_interaction_command(ctx: CommandInteraction) -> anyhow::Result<()> {
        Self::new(ctx.data.clone())?.interaction_command(ctx).await
    }

    /// Execute a component interaction. This could be a button or other form of interaciton
    async fn run_interaction_component(ctx: ComponentInteraction) -> anyhow::Result<()> {
        let interaction = match ctx
            .database
            .get_interaction_by_message_id(ctx.message.id.get() as i64)
            .await?
        {
            Some(interaction) => interaction,
            None => {
                warn!(ctx = ?ctx, "Attempting to handle component with an interaction that does not exist in the database");
                return Ok(());
            }
        };

        let data = match interaction.data.clone().map(|x|x.0) {
            Some(InteractionData::ApplicationCommand(data)) => data,
            _ => {
                return Err(anyhow!(
                    "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                    interaction.data
                ))
            }
        };

        Self::new(data)?.interaction_component(ctx).await
    }

    /// Execute a modal interaction
    async fn run_interaction_modal(ctx: ModalInteraction) -> anyhow::Result<()> {
        Self::interaction_modal(ctx).await
    }

    /// Execute the handler for an autocomplete context
    async fn run_interaction_autocomplete(ctx: CommandInteraction) -> anyhow::Result<()> {
        Self::interaction_autocomplete(ctx).await
    }
}

pub trait ExecuteLuroCommand: Sized {
    /// The function to execute for the command / command group
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Handle a component interaction. This could be a button or other form of interaciton
    async fn interaction_component(self, ctx: ComponentInteraction) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Create and respond to a button interaction
    async fn interaction_modal(ctx: ModalInteraction) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }

    /// Create and respond to a button interaction
    async fn interaction_autocomplete(ctx: CommandInteraction) -> anyhow::Result<()> {
        ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await
    }
}
