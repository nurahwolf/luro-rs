use luro_model::{response::SimpleResponse, types::CommandResponse};

use crate::{CommandInteraction, ComponentInteraction, ModalInteraction};

pub trait LuroCommand {
    /// The function to execute for the command / command group
    fn interaction_command(self, ctx: CommandInteraction) -> impl std::future::Future<Output = anyhow::Result<CommandResponse>> + Send
    where
        Self: Sized,
    {
        async move { ctx.simple_response(SimpleResponse::UnknownCommand(ctx.command_name())).await }
    }

    /// Handle a component interaction. This could be a button or other form of interaciton
    fn interaction_component(
        self,
        ctx: ComponentInteraction,
        _invoking_interaction: twilight_model::application::interaction::Interaction,
    ) -> impl std::future::Future<Output = anyhow::Result<CommandResponse>> + Send
    where
        Self: Sized,
    {
        async move { ctx.simple_response(SimpleResponse::UnknownCommand(ctx.command_name())).await }
    }

    /// Create and respond to a button interaction
    fn interaction_modal(ctx: ModalInteraction) -> impl std::future::Future<Output = anyhow::Result<CommandResponse>> + Send {
        async move { ctx.simple_respponse(SimpleResponse::UnknownCommand(ctx.command_name())).await }
    }

    /// Create and respond to a button interaction
    fn interaction_autocomplete(ctx: CommandInteraction) -> impl std::future::Future<Output = anyhow::Result<CommandResponse>> + Send {
        async move { ctx.simple_response(SimpleResponse::UnknownCommand(ctx.command_name())).await }
    }
}
