use crate::Command;

mod command_adminabuse;
mod command_register;
mod command_saveconfig;
mod command_shutdown;

pub fn commands() -> [Command; 4] {
    [
        command_adminabuse::adminabuse(),
        command_register::register(),
        command_saveconfig::save_config(),
        command_shutdown::shutdown()
    ]
}
