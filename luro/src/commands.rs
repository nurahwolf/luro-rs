use std::collections::HashMap;

use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

use self::{
    about::AboutCommand, boop::BoopCommand, count::CountCommand, heck::HeckCommands,
    hello::HelloCommand, moderator::ModeratorCommands, music::MusicCommands, owner::OwnerCommands,
    say::SayCommand,
};

pub mod about;
pub mod boop;
pub mod count;
pub mod heck;
pub mod hello;
pub mod moderator;
pub mod music;
pub mod owner;
pub mod say;

#[derive(Default)]
pub struct Commands {
    /// Commands that are available to be registered within guilds
    pub guild_commands: HashMap<&'static str, Command>,
    /// Commands that are available to be registered globally
    pub global_commands: HashMap<&'static str, Command>,
}

impl Commands {
    /// Return a default set of commands to register
    pub fn default_commands() -> Self {
        // Create the hashmaps
        let mut init = Self {
            guild_commands: Default::default(),
            global_commands: Default::default(),
        };

        // Add some default commands
        init.global_commands
            .insert("hello", HelloCommand::create_command().into());
        init.global_commands
            .insert("count", CountCommand::create_command().into());
        init.global_commands
            .insert("say", SayCommand::create_command().into());
        init.global_commands
            .insert("mod", ModeratorCommands::create_command().into());
        init.global_commands
            .insert("music", MusicCommands::create_command().into());
        init.global_commands
            .insert("boop", BoopCommand::create_command().into());
        init.global_commands
            .insert("heck", HeckCommands::create_command().into());
        init.global_commands
            .insert("owner", OwnerCommands::create_command().into());
        init.global_commands
            .insert("about", AboutCommand::create_command().into());

        // Return our initialised commands
        init
    }
}
