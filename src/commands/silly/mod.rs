use crate::Command;

mod command_boop;
mod command_filedetails;
mod command_oracle;
mod commands_vote;

pub fn commands() -> [Command; 5] {
    [
        command_boop::boop(),
        command_filedetails::file_details(),
        command_oracle::oracle(),
        commands_vote::vote(),
        commands_vote::getvotes()
    ]
}
