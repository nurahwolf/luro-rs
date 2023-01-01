use crate::Command;

mod command_deafen;
mod command_join;
mod command_leave;
mod command_mute;
mod command_play;
mod command_playfaded;
mod command_skip;
mod command_stop;
mod command_undeafen;
mod command_unmute;
mod command_volume;
mod command_nowplaying;
mod struct_music;

pub fn commands() -> [Command; 12] {
    [
        command_deafen::deafen(),
        command_join::join(),
        command_leave::leave(),
        command_mute::mute(),
        command_play::play(),
        command_playfaded::playfaded(),
        command_skip::skip(),
        command_stop::stop(),
        command_undeafen::undeafen(),
        command_unmute::unmute(),
        command_volume::volume(),
        command_nowplaying::nowplaying()
    ]
}
