use twilight_model::{
    application::command::{Command, CommandOption, CommandOptionType},
    id::Id,
};

pub type SlashResult = Result<(), SlashError>;
pub type SlashError = ();

pub struct SlashCommandParameter {
    pub name: String,
    pub description: String,
    pub required: bool,
    // pub choices: Vec<CommandParameterChoice>,
}

/// Implements an interaction command.
pub struct SlashCommand {
    // pub command: Box<dyn FnOnce() -> BoxFuture<'static, SlashResult> + Send>,
    // pub command: for<'a> fn(SlashContext) -> BoxFuture<'static, SlashResult>,
    // pub subcommand: Vec<SlashCommand>,
    pub name: String,
    pub description: String,
    pub long_description: Option<String>,
    pub nsfw: bool,
    pub ephemeral: bool,
    pub parameters: Vec<SlashCommandParameter>,
    // pub checks: Vec<fn() -> BoxFuture<'static, Result<bool, SlashError>>>,
}

impl SlashCommand {
    pub fn twilight_command(&self) -> Command {
        let options = self
            .parameters
            .iter()
            .map(|option| CommandOption {
                autocomplete: None,
                channel_types: None,
                choices: None,
                description: option.description.clone(),
                description_localizations: None,
                kind: CommandOptionType::String,
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: option.name.clone(),
                name_localizations: None,
                options: None,
                required: Some(option.required),
            })
            .collect();

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
            options,
            version: Id::new(1),
        }
    }
}
