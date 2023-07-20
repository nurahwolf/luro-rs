use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "count", desc = "Test to see if the framework is globally mutable")]
pub struct CountCommand {}

#[async_trait]
impl LuroCommand for CountCommand {
    async fn run_command(self, _interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let message;

        {
            let mut global_data = ctx.global_data.write();
            global_data.count += 1;
            message = format!("Here is your number: {}", global_data.count);
        }

        Ok(InteractionResponse::Content {
            content: message,
            ephemeral: false,
            deferred: false
        })
    }
}
