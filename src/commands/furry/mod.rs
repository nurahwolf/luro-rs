use crate::Command;

mod command_e621;
mod command_fursona;
mod commands_uwuify;
pub(crate) mod struct_e621;

pub fn commands() -> [Command; 4] {
    [command_e621::e621(), command_fursona::fursona(), commands_uwuify::uwu(), commands_uwuify::uwuify()]
}
