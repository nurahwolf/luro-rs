use tracing::info;
use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

use crate::{
    interactions::{about::AboutCommand, hello_world::HelloCommand, say::SayCommand, heck::HeckCommand},
    Luro,
};

pub mod join;
pub mod leave;
pub mod pause;
pub mod play;
pub mod seek;
pub mod stop;
pub mod volume;

/// A structure containing all of our commands, as well as some utility functions relating to thems
#[derive(Clone, Debug, PartialEq)]
pub struct LuroCommands {
    pub global_commands: Vec<Command>,
    pub guild_commands: Vec<Command>,
}

impl Luro {
    pub fn set_default_commands() -> LuroCommands {
        info!("Registered default commands");
        LuroCommands {
            global_commands: vec![
                HelloCommand::create_command().into(),
                AboutCommand::create_command().into(),
                SayCommand::create_command().into(),
                HeckCommand::create_command().into()
            ],
            guild_commands: vec![],
        }
    }

    pub async fn register_global_commands(&self) -> anyhow::Result<()> {
        let global_commands = match self.commands.read() {
            Ok(commands) => commands.global_commands.clone(),
            Err(why) => panic!("Command Mutex is poisoned: {why}"),
        };

        self.http
            .interaction(self.application_id)
            .set_global_commands(&global_commands)
            .await?
            .model()
            .await?;

        Ok(())
    }

    // fn command_id(command_name: &str, commands: &[Command]) -> Option<Id<CommandMarker>> {
    //     commands
    //         .iter()
    //         .find_map(|command| (command.name == command_name).then_some(command.id?))
    // }

    // fn new(commands: &[Command]) -> Result<Self> {
    //     Ok(Self {
    //         hello: Self::command_id("hello", commands)?,
    //         clickme: Self::command_id("clickme", commands)?,
    //     })
    // }
}
