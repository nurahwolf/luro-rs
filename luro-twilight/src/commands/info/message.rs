use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "message", desc = "Information about a message")]
pub struct Message {
    /// The message ID to fetch
    message_id: String,
}
