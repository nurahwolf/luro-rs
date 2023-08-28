use anyhow::anyhow;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::CommandModel;
use twilight_model::{application::interaction::application_command::CommandData, guild::Permissions};

use crate::responses::SimpleResponse;
use crate::InteractionComponent;
use crate::{Framework, InteractionCommand, InteractionModal};

/// Add some custom functionality around [CommandModel]
pub trait LuroCommand: CommandModel {
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
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        SimpleResponse::UnknownCommand(&interaction.data.name)
            .respond(&ctx, &interaction)
            .await
    }

    /// Handle a component interaction. This could be a button or other form of interaciton
    async fn handle_component<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionComponent
    ) -> anyhow::Result<()> {
        SimpleResponse::UnknownCommand(&interaction.data.custom_id)
            .respond(&ctx, &interaction)
            .await
    }

    /// Create and respond to a button interaction
    async fn handle_modal<D: LuroDatabaseDriver>(ctx: Framework<D>, interaction: InteractionModal) -> anyhow::Result<()> {
        SimpleResponse::UnknownCommand(&interaction.data.custom_id)
            .respond(&ctx, &interaction)
            .await
    }

    /// The default permissions a user needs to run this command
    fn default_permissions() -> Permissions {
        Permissions::all()
    }
}
