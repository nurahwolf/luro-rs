use luro_model::database::Database;
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    id::{marker::InteractionMarker, Id},
};

use crate::responses::StandardResponse;

use super::interaction::{InteractionContext, InteractionError, InteractionResult};

pub trait CreateCommand: twilight_interactions::command::CommandModel + twilight_interactions::command::CreateCommand {
    fn cmd(interaction: &Interaction) -> Result<Self, InteractionError> {
        let interaction_data = match interaction.data.clone() {
            Some(interaction_data) => interaction_data,
            None => return Err(InteractionError::NoApplicationData),
        };

        let command_data = match interaction_data {
            InteractionData::ApplicationCommand(command_data) => *command_data,
            _ => return Err(InteractionError::NoApplicationData),
        };

        match Self::from_interaction(command_data.into()) {
            Ok(cmd) => Ok(cmd),
            Err(why) => Err(InteractionError::ParseError(why)),
        }
    }

    /// An internal function which parses the command data into the type, then forwards it to handle_interaction
    fn interaction_handler(f: &mut InteractionContext) -> impl std::future::Future<Output = InteractionResult<()>> + Send
    where
        Self: Send,
    {
        async {
            match f.interaction.kind {
                InteractionType::Ping => no_handler("ping"),
                InteractionType::ApplicationCommand => Self::cmd(&f.interaction)?.handle_command(f).await,
                InteractionType::MessageComponent => Self::handle_component(f).await,
                InteractionType::ApplicationCommandAutocomplete => no_handler("autocomplete"),
                InteractionType::ModalSubmit => Self::handle_modal(f).await,
                unknown_kind => no_handler(unknown_kind.kind()),
            }
        }
    }

    fn command_from_component(
        framework: &InteractionContext,
    ) -> impl std::future::Future<Output = Result<Self, InteractionError>> + Send {
        async {
            let message = framework.compontent_message()?;
            let interaction_id = match message.interaction.as_ref() {
                Some(interaction) => interaction.id,
                None => match message.referenced_message.as_ref() {
                    Some(message) => match message.interaction.as_ref() {
                        Some(interaction) => interaction.id,
                        None => return Err(InteractionError::CommandFromComponent),
                    },
                    None => return Err(InteractionError::CommandFromComponent),
                },
            };

            Self::from_interaction_id(&framework.gateway.database, interaction_id).await
        }
    }

    /// Use the database to fetch an interaction
    fn from_interaction_id(
        db: &Database,
        id: Id<InteractionMarker>,
    ) -> impl std::future::Future<Output = Result<Self, InteractionError>> + Send {
        async move {
            let interaction = db.fetch_interaction(id).await?;
            Self::cmd(&interaction)
        }
    }

    fn handle_component(framework: &mut InteractionContext) -> impl std::future::Future<Output = InteractionResult<()>> + Send {
        async {
            let response = StandardResponse::UnknownCommand(framework.command_name());
            framework.standard_response(response).await
        }
    }

    fn handle_modal(framework: &mut InteractionContext) -> impl std::future::Future<Output = InteractionResult<()>> + Send {
        async {
            let response = StandardResponse::UnknownCommand(framework.command_name());
            framework.standard_response(response).await
        }
    }

    fn handle_autocomplete(framework: &mut InteractionContext) -> impl std::future::Future<Output = InteractionResult<()>> + Send {
        async {
            let response = StandardResponse::UnknownCommand(framework.command_name());
            framework.standard_response(response).await
        }
    }

    fn handle_command(self, framework: &mut InteractionContext) -> impl std::future::Future<Output = InteractionResult<()>> + Send {
        async {
            let response = StandardResponse::UnknownCommand(framework.command_name());
            framework.standard_response(response).await
        }
    }

    fn setup_command() -> twilight_model::application::command::Command {
        Self::create_command().into()
    }
}

fn no_handler(handler: &str) -> InteractionResult<()> {
    tracing::info!("Received data for {handler} handler, which is not configured!");
    Ok(())
}
