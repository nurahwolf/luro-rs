use crate::Command;

mod e621;
mod furaffinity;
mod fursona;
mod uwuify;
mod reverse;

pub fn commands() -> [Command; 7] {
    [e621::e621(), fursona::fursona(), uwuify::uwu(), uwuify::uwuify(), furaffinity::fa(), reverse::image_source(), reverse::source_context()]
}
