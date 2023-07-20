use anyhow::Error;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::interactions::InteractionResponse;

#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct SayCommand {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>
}

impl SayCommand {
    pub async fn run(self) -> Result<InteractionResponse, Error> {
        let message = if let Some(user) = self.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, self.message)
        } else {
            self.message
        };

        Ok(InteractionResponse::Content {
            content: message,
            ephemeral: false,
            deferred: false
        })
    }
}
