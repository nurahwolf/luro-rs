use core::fmt;
use std::sync::RwLock;

use twilight_model::application::command::Command;

use self::hecks::Hecks;

pub mod hecks;
pub mod luro;

/// Guild specific data
pub struct GuildSettings {
    /// A vector of commands that are loaded in a guild
    pub global_commands: RwLock<Vec<Command>>,
}

pub struct UserSettings {}

/// Data for global commands
pub struct GlobalCommands {
    /// Global hecks! Woo
    pub global_hecks: RwLock<Hecks>,
}

#[derive(Debug)]
pub enum LuroError {
    NoInteractionData,
    NoApplicationCommand,
    NoMessageInteractionData,
    NoApplicationData,
}

impl std::error::Error for LuroError {}

impl fmt::Display for LuroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuroError::NoMessageInteractionData => write!(f, "No Message Interaction Data"),
            LuroError::NoInteractionData => write!(f, "No data was found in the interaction"),
            LuroError::NoApplicationCommand => write!(
                f,
                "No ApplicationCommand was found in the interaction's data"
            ),
            LuroError::NoApplicationData => {
                write!(f, "Unable to get data from the application rwlock")
            } // _ => write!(f, "Error description not written yet"),
        }
    }
}
