use crate::Command;

mod command_db_add;
mod command_db_get;
mod command_db_total;
mod command_e621tiled;
mod command_heck;
mod command_reuseresponse;

pub fn commands() -> [Command; 7] {
    [
        command_e621tiled::e621_tiled(),
        command_reuseresponse::test_reuse_response(),
        command_heck::heck(),
        command_heck::heck_user(),
        command_db_add::db_add(),
        command_db_get::db_get(),
        command_db_total::db_total()
    ]
}
