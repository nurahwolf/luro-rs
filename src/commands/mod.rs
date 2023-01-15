use crate::Command;

mod api;
mod dice;
pub mod furry;
mod guild;
mod moderator;
mod music;
mod owner;
mod quote;
mod silly;
mod simple;
mod testing;
mod luro;

pub fn commands() -> Vec<Command> {
    owner::commands()
        .into_iter()
        .chain(music::commands())
        .chain(simple::commands())
        .chain(moderator::commands())
        .chain(guild::commands())
        .chain(quote::commands())
        .chain(furry::commands())
        .chain(silly::commands())
        .chain(api::commands())
        .chain(dice::commands())
        .chain(testing::commands())
        .chain(luro::commands())
        .collect()
}
