use anyhow::Error;
use luro_framework::{CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "purge", desc = "Remove up to 100 messages from a channel")]
pub struct Purge {
    /// Choose how many messages should be removed
    #[command(min_value = 1, max_value = 100)]
    amount: i64,
}

impl LuroCommand for Purge {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let messages = ctx
            .twilight_client
            .channel_messages(ctx.channel.id)
            .limit(self.amount as u16)
            .await?
            .model()
            .await?;

        if self.amount == 1 {
            ctx.twilight_client
                .delete_message(ctx.channel.id, messages.first().ok_or_else(|| Error::msg("No messages found"))?.id)
                .await?;
        } else {
            let message_ids = messages.into_iter().map(|messages| messages.id).collect::<Vec<_>>();
            ctx.twilight_client.delete_messages(ctx.channel.id, &message_ids).await?;
        }

        ctx.respond(|r| r.content("Done!!").ephemeral()).await
    }
}
