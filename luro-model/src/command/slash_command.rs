use twilight_model::{application::command::Command, id::Id};

use crate::BoxFuture;

pub type SlashResult = Result<(), SlashError>;
pub type SlashError = ();

/// Implements an interaction command.
pub struct SlashCommand {
    pub command: Box<dyn FnOnce() -> BoxFuture<'static, SlashResult> + Send>,
    // pub subcommand: Vec<SlashCommand>,
    pub name: String,
    pub description: String,
    pub long_description: Option<String>,
    pub nsfw: bool,
    // pub checks: Vec<fn() -> BoxFuture<'static, Result<bool, SlashError>>>,
}

impl SlashCommand {
    pub fn twilight_command(&self) -> Command {
        let version = Id::new(1);
        Command {
            application_id: None,
            default_member_permissions: None,
            dm_permission: None,
            description: self.description.clone(),
            description_localizations: None,
            guild_id: None,
            id: None,
            kind: twilight_model::application::command::CommandType::ChatInput,
            name: self.name.clone(),
            name_localizations: None,
            nsfw: Some(self.nsfw),
            options: vec![],
            version,
        }
    }
}
