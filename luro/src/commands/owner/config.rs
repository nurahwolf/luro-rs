
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "config",
    desc = "Print out information about my configuration. You can override some options at runtime too."
)]
pub struct ConfigCommand {}


impl LuroCommand for ConfigCommand {}
