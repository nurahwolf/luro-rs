use crate::Command;

mod db_add;
mod db_get;
mod db_total;
mod e621tiled;
mod heck;
mod reuseresponse;

pub fn commands() -> [Command; 7] {
    [
        e621tiled::e621_tiled(),
        reuseresponse::test_reuse_response(),
        heck::heck(),
        heck::heck_user(),
        db_add::db_add(),
        db_get::db_get(),
        db_total::db_total()
    ]
}
