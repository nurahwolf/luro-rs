use crate::Command;

mod lights;
mod lodestonenews;
mod printerfacts;
mod twitter;
mod xkcd;
mod urban;

pub fn commands() -> [Command; 7] {
    [
        lights::lights(),
        lodestonenews::lodestonenews(),
        printerfacts::printerfacts(),
        twitter::twitter(),
        xkcd::xkcd(),
        urban::urban(),
        urban::random_urban()
    ]
}
