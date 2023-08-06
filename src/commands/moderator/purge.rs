use std::convert::TryInto;

use anyhow::Error;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::guild::Permissions;

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "purge",
    desc = "Remove up to 100 messages from a channel",
    default_permissions = "Self::default_permissions"
)]
pub struct PurgeCommand {
    /// Choose how many messages should be removed
    #[command(min_value = 1, max_value = 100)]
    amount: i64
}

#[async_trait]
impl LuroCommand for PurgeCommand {
    fn default_permissions() -> Permissions {
        Permissions::MANAGE_MESSAGES
    }

    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let channel = ctx.channel(&slash)?;

        if self.amount == 1 {
            let message = ctx
                .twilight_client
                .channel_messages(channel.id)
                .limit(1)
                .await?
                .model()
                .await?;
            ctx.twilight_client
                .delete_message(channel.id, message.first().ok_or_else(|| Error::msg("No messages found"))?.id)
                .await?;
        } else {
            let messages = ctx
                .twilight_client
                .channel_messages(channel.id)
                .limit(self.amount.try_into().unwrap())
                .await?
                .model()
                .await?;
            let message_ids = messages.into_iter().map(|messages| messages.id).collect::<Vec<_>>();
            ctx.twilight_client.delete_messages(channel.id, &message_ids).await?;
        }

        slash.content("Done!!".to_owned()).ephemeral();
        ctx.respond(&mut slash).await
    }
}
