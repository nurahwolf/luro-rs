use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::ApplicationCommandData;

use crate::{Framework, InteractionCommand, InteractionComponent, InteractionModal};

pub type CommandResult = std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + 'static + Send>>;

/// A structure containing a standard command implementation, for progmatically loading commands
#[derive(Clone)]
pub struct LuroCommand<D: LuroDatabaseDriver> {
    /// The name of the command
    pub name: &'static str,
    /// The core [ApplicationCommandData] needed to create the command in Discord
    pub create: fn() -> ApplicationCommandData,
    /// Command to execute in an interaction context
    pub interaction_command: fn(Framework<D>, InteractionCommand) -> CommandResult,
    /// A component to execute
    pub component: fn(Framework<D>, InteractionComponent) -> CommandResult,
    /// A modal to execute
    pub modal: fn(Framework<D>, InteractionModal) -> CommandResult,
    /// A autocomplete to execute
    pub autocomplete: fn(Framework<D>, InteractionCommand) -> CommandResult,
}
