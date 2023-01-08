use crate::Command;

mod e621;
mod furaffinity;
mod fursona;
mod image_source;
mod uwuify;

pub fn commands() -> [Command; 7] {
    [
        e621::e621(),
        fursona::fursona(),
        uwuify::uwu(),
        uwuify::uwuify(),
        furaffinity::fa(),
        image_source::saucenao_lookup(),
        image_source::saucenao_context()
    ]
}
