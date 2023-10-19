use twilight_interactions::command::ApplicationCommandData;

use crate::{CommandInteraction, ComponentInteraction, ModalInteraction};

pub type CommandResult = std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + 'static + Send>>;

/// A structure containing a standard command implementation, for progmatically loading commands
#[derive(Clone, Debug)]

// TODO: Try Remove
pub struct LuroCommand {
    /// The name of the command
    pub name: &'static str,
    /// The core [ApplicationCommandData] needed to create the command in Discord
    pub create: fn() -> ApplicationCommandData,
    /// Command to execute in an interaction context
    pub interaction_command: fn(CommandInteraction) -> CommandResult,
    /// A component to execute
    pub component: fn(ComponentInteraction) -> CommandResult,
    /// A modal to execute
    pub modal: fn(ModalInteraction) -> CommandResult,
    /// A autocomplete to execute
    pub autocomplete: fn(CommandInteraction) -> CommandResult,
}