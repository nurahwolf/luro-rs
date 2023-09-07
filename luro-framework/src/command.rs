use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{application::interaction::application_command::CommandData, guild::Permissions};

use crate::responses::SimpleResponse;
use crate::slash_command::LuroCommand;
use crate::{Framework, InteractionCommand, InteractionComponent, InteractionModal};

/// Simply a wrapper around [LuroCommand], ensuring that [CreateCommand] is present
pub trait LuroCommandBuilder: LuroCommandTrait + CreateCommand {
    fn new_command<D: LuroDatabaseDriver + 'static>() -> LuroCommand<D> {
        LuroCommand {
            name: Self::NAME,
            create: Self::create_command,
            interaction_command: Self::handle_interaction,
            component: Self::handle_component,
            modal: Self::handle_modal,
            autocomplete: Self::handle_autocomplete
        }
    }
}

/// Add some custom functionality around [CommandModel]
#[async_trait]
pub trait LuroCommandTrait: CommandModel {
    /// Create a new command and get it's data from the interaction
    fn new(data: Box<CommandData>) -> anyhow::Result<Self> {
        let data = *data;
        match Self::from_interaction(data.into()) {
            Ok(ok) => Ok(ok),
            Err(why) => Err(anyhow!(
                "Got interaction data, but failed to parse it to the command type specified: {why}"
            ))
        }
    }

    /// Run the command / command group
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Arc<Framework<D>>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        SimpleResponse::unknown_command(&ctx, &interaction).await
    }

    /// Handle a component interaction. This could be a button or other form of interaciton
    async fn handle_component<D: LuroDatabaseDriver>(
        ctx: Arc<Framework<D>>,
        interaction: InteractionComponent
    ) -> anyhow::Result<()> {
        SimpleResponse::UnknownCommand(&interaction.data.custom_id)
            .respond(&ctx, &interaction)
            .await
    }

    /// Create and respond to a button interaction
    async fn handle_modal<D: LuroDatabaseDriver>(ctx: Arc<Framework<D>>, interaction: InteractionModal) -> anyhow::Result<()> {
        SimpleResponse::UnknownCommand(&interaction.data.custom_id)
            .respond(&ctx, &interaction)
            .await
    }

    /// Create and respond to a button interaction
    async fn handle_autocomplete<D: LuroDatabaseDriver>(
        ctx: Arc<Framework<D>>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        SimpleResponse::UnknownCommand(&interaction.data.name)
            .respond(&ctx, &interaction)
            .await
    }

    /// The default permissions a user needs to run this command
    fn default_permissions() -> Permissions {
        Permissions::all()
    }
}
