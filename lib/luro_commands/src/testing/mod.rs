use crate::Command;

mod heck;
mod reuseresponse;

pub fn commands() -> [Command; 3] {
    [reuseresponse::test_reuse_response(), heck::heck(), heck::heck_user()]
}
