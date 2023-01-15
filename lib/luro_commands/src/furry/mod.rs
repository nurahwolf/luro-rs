use crate::Command;

mod fursona;
mod image_source;
mod uwuify;

pub fn commands() -> [Command; 5] {
    [
        fursona::fursona(),
        uwuify::uwu(),
        uwuify::uwuify(),
        image_source::saucenao_lookup(),
        image_source::saucenao_context()
    ]
}
