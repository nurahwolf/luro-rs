use crate::Command;

mod command_e621;
mod command_furaffinity;
mod command_fursona;
mod commands_uwuify;
pub mod function_fa;
pub(crate) mod struct_e621;
pub(crate) mod struct_furaffinity;

pub fn commands() -> [Command; 5] {
    [
        command_e621::e621(),
        command_fursona::fursona(),
        commands_uwuify::uwu(),
        commands_uwuify::uwuify(),
        command_furaffinity::fa()
    ]
}
