use crate::Command;

mod e621;
mod furaffinity;
mod fursona;
mod uwuify;

pub fn commands() -> [Command; 5] {
    [
        e621::e621(),
        fursona::fursona(),
        uwuify::uwu(),
        uwuify::uwuify(),
        furaffinity::fa()
    ]
}
