use crate::Command;

mod command_lights;
mod command_lodestonenews;
mod command_printerfacts;
mod command_twitter;
mod command_xkcd;
mod commands_urban;
mod struct_twitter;

pub fn commands() -> [Command; 7] {
    [
        command_lights::lights(),
        command_lodestonenews::lodestonenews(),
        command_printerfacts::printerfacts(),
        command_twitter::twitter(),
        command_xkcd::xkcd(),
        commands_urban::urban(),
        commands_urban::random_urban()
    ]
}
