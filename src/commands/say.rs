use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::interaction::Interaction;

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct SayCommand {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>
}

#[async_trait]
impl LuroCommand for SayCommand {
    async fn run_command(self, _interaction: Interaction, _ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let message = if let Some(user) = self.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, self.message)
        } else {
            self.message
        };

        Ok(InteractionResponse::Content {
            content: message,
            luro_response: Default::default()
        })
    }
}
