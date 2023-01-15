use crate::Command;

mod boop;
mod filedetails;
mod oracle;

pub fn commands() -> [Command; 3] {
    [boop::boop(), filedetails::file_details(), oracle::oracle()]
}
