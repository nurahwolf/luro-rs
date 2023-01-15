use luro_core::Command;

mod deafen;
mod join;
mod leave;
mod mute;
mod nowplaying;
mod play;
mod playfaded;
mod shuffle;
mod skip;
mod stop;
mod undeafen;
mod unmute;
mod volume;

pub fn songbird_commands() -> [Command; 13] {
    [
        deafen::deafen(),
        join::join(),
        leave::leave(),
        mute::mute(),
        play::play(),
        playfaded::playfaded(),
        skip::skip(),
        stop::stop(),
        undeafen::undeafen(),
        unmute::unmute(),
        volume::volume(),
        nowplaying::nowplaying(),
        shuffle::shuffle()
    ]
}
