use twilight_model::application::command::Command;

use super::{CommandFlags, CommandKind};

impl CommandKind {
    pub fn create(&self) -> Command {
        match self {
            CommandKind::Chat(cmd) => (cmd.create)().into(),
            CommandKind::Message(cmd) => (cmd.create)()
        }
    }

    pub fn flags(&self) -> CommandFlags {
        match self {
            CommandKind::Chat(cmd) => cmd.flags,
            CommandKind::Message(cmd) => cmd.flags
        }
    }
}
