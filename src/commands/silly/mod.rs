use crate::Command;

mod boop;
mod filedetails;
mod oracle;
mod vote;

pub fn commands() -> [Command; 5] {
    [
        boop::boop(),
        filedetails::file_details(),
        oracle::oracle(),
        vote::vote(),
        vote::getvotes()
    ]
}
