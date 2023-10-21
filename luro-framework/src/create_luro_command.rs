use anyhow::{anyhow, Context};
use luro_database::DatabaseInteraction;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::CommandData;

use crate::standard_response::Response;
use crate::{CommandInteraction, ComponentInteraction, InteractionContext, LuroCommand, ModalInteraction};

pub trait CreateLuroCommand: CommandModel + CreateCommand {
    fn handle_interaction(interaction: InteractionContext) -> impl std::future::Future<Output = anyhow::Result<()>> + Send
    where
        Self: Send,
    {
        async move {
            match interaction.clone() {
                InteractionContext::Command(ctx) => Self::new(ctx.data.clone())?.interaction_command(ctx).await,
                InteractionContext::CommandAutocomplete(ctx) => Self::interaction_autocomplete(ctx).await,
                InteractionContext::Component(ctx) => {
                    let raw_interaction = ctx
                        .message
                        .interaction
                        .as_ref()
                        .context("Expected message to have interaction data")?;
                    let interaction = ctx
                        .database
                        .get_interaction(raw_interaction.id.get() as i64)
                        .await?
                        .context("Database does not contain this interaction")?;
                    let data = interaction
                        .data
                        .as_ref()
                        .map(|x| x.0.clone())
                        .context("Expected interaction recorded in database to contain interaction data")?;
                    let command_data = match data {
                        twilight_model::application::interaction::InteractionData::ApplicationCommand(data) => data,
                        data => return Err(anyhow!("Unexpected data returned: '{:#?}'", data)),
                    };

                    Self::new(command_data)?.interaction_component(ctx, interaction).await
                }
                InteractionContext::Modal(ctx) => Self::interaction_modal(ctx).await,
            }
        }
    }

    fn setup_command() -> Command {
        Self::create_command().into()
    }

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
    fn interaction_command(self, ctx: CommandInteraction) -> impl std::future::Future<Output = anyhow::Result<()>> + Send
    where
        Self: Send,
    {
        async move { ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await }
    }

    /// Execute a component interaction. This could be a button or other form of interaciton
    fn interaction_component(
        self,
        ctx: ComponentInteraction,
        _: DatabaseInteraction,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send
    where
        Self: Send,
    {
        async move { ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await }
    }

    /// Execute a modal interaction
    fn interaction_modal(ctx: ModalInteraction) -> impl std::future::Future<Output = anyhow::Result<()>> + Send
    where
        Self: Send,
    {
        async move { ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await }
    }

    /// Execute the handler for an autocomplete context
    fn interaction_autocomplete(ctx: CommandInteraction) -> impl std::future::Future<Output = anyhow::Result<()>> + Send
    where
        Self: Send,
    {
        async move { ctx.response_simple(Response::UnknownCommand(ctx.command_name())).await }
    }
}

impl<T: CreateLuroCommand + Send> LuroCommand for T {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        Self::interaction_command(self, ctx).await
    }

    async fn interaction_component(self, ctx: ComponentInteraction, db: luro_database::DatabaseInteraction) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        Self::interaction_component(self, ctx, db).await
    }

    async fn interaction_modal(ctx: ModalInteraction) -> anyhow::Result<()> {
        Self::interaction_modal(ctx).await
    }

    async fn interaction_autocomplete(ctx: CommandInteraction) -> anyhow::Result<()> {
        Self::interaction_autocomplete(ctx).await
    }
}
