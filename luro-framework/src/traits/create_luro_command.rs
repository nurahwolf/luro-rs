use anyhow::anyhow;
use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::InteractionData;

use crate::responses::Response;
use crate::{CommandInteraction, ComponentInteraction, ModalInteraction, ExecuteLuroCommand};

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
    fn run_interaction_command(ctx: CommandInteraction) -> impl std::future::Future<Output = anyhow::Result<()>> + Send
    where
        Self: Send,
    {
        async { Self::new(ctx.data.clone())?.interaction_command(ctx).await }
    }

    /// Execute a component interaction. This could be a button or other form of interaciton
    fn run_interaction_component(ctx: ComponentInteraction) -> impl std::future::Future<Output = anyhow::Result<()>> + Send
    where
        Self: Send,
    {
        async {
            let raw_interaction = match ctx.message.interaction.as_ref() {
                Some(interaction) => interaction,
                None => {
                    return ctx
                        .response_simple(Response::InternalError(anyhow!(
                            "Message does not have an interaction recorded in my database"
                        )))
                        .await
                }
            };

            let interaction = match ctx.database.get_interaction(raw_interaction.id.get() as i64).await? {
                Some(interaction) => interaction,
                None => {
                    return ctx
                        .response_simple(Response::InternalError(anyhow!(
                            "Database does not contain an interaction with ID '{}'",
                            raw_interaction.id
                        )))
                        .await
                }
            };

            let data = match interaction.data.as_ref().map(|x| x.0.clone()) {
                Some(InteractionData::ApplicationCommand(data)) => data,
                _ => {
                    return ctx
                        .response_simple(Response::InternalError(anyhow!(
                            "Interaction '{}' does not contain ApplicationCommandData",
                            raw_interaction.id
                        )))
                        .await
                }
            };

            Self::new(data)?.interaction_component(ctx, interaction).await
        }
    }

    /// Execute a modal interaction
    fn run_interaction_modal(ctx: ModalInteraction) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        async { Self::interaction_modal(ctx).await }
    }

    /// Execute the handler for an autocomplete context
    fn run_interaction_autocomplete(ctx: CommandInteraction) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        async { Self::interaction_autocomplete(ctx).await }
    }
}