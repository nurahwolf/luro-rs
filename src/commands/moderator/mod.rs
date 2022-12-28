use crate::Command;

mod command_cleanup;
mod command_deletebotmessage;
mod command_punish;

pub fn commands() -> [Command; 3] {
    [command_cleanup::cleanup(), command_deletebotmessage::delete_botmessage(), command_punish::punish()]
}
