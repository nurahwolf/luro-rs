use crate::Command;

mod command_e621tiled;
mod command_heck;
mod command_reuseresponse;

pub fn commands() -> [Command; 4] {
    [
        command_e621tiled::e621_tiled(),
        command_reuseresponse::test_reuse_response(),
        command_heck::heck(),
        command_heck::heck_user()
    ]
}
