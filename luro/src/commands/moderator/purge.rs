use std::convert::TryInto;

use anyhow::Error;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{functions::interaction_context, interactions::InteractionResponse, LuroContext, SlashResponse};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "purge", desc = "Remove up to 100 messages from a channel")]
pub struct PurgeCommand {
    /// Choose how many messages should be removed
    #[command(min_value = 1, max_value = 100)]
    amount: i64
}

impl PurgeCommand {
    pub async fn run(self, ctx: &LuroContext, interaction: &Interaction) -> SlashResponse {
        let ephemeral = ctx.defer_interaction(interaction, true).await?;

        let (interaction_channel, _, _) = interaction_context(interaction, "mod purge")?;

        if self.amount == 1 {
            let message = ctx
                .twilight_client
                .channel_messages(interaction_channel.id)
                .limit(1)?
                .await?
                .model()
                .await?;
            ctx.twilight_client
                .delete_message(
                    interaction_channel.id,
                    message.first().ok_or_else(|| Error::msg("No messages found"))?.id
                )
                .await?;
        } else {
            let messages = ctx
                .twilight_client
                .channel_messages(interaction_channel.id)
                .limit(self.amount.try_into().unwrap())?
                .await?
                .model()
                .await?;
            let message_ids = messages.into_iter().map(|messages| messages.id).collect::<Vec<_>>();
            ctx.twilight_client
                .delete_messages(interaction_channel.id, &message_ids)?
                .await?;
        }

        Ok(InteractionResponse::Content {
            content: "Done!!".to_owned(),
            ephemeral
        })
    }
}
