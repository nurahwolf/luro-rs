use anyhow::Error;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::guild::Permissions;

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "purge",
    desc = "Remove up to 100 messages from a channel",
    default_permissions = "Self::default_permissions"
)]
pub struct PurgeCommand {
    /// Choose how many messages should be removed
    #[command(min_value = 1, max_value = 100)]
    amount: i64,
}

impl LuroCommand for PurgeCommand {
    fn default_permissions() -> Permissions {
        Permissions::MANAGE_MESSAGES
    }

    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let channel = ctx.interaction.channel.as_ref().unwrap().id;
        let twilight = &ctx.framework.twilight_client;
        let messages = twilight
            .channel_messages(channel)
            .limit(self.amount as u16)
            .await?
            .model()
            .await?;

        if self.amount == 1 {
            twilight
                .delete_message(channel, messages.first().ok_or_else(|| Error::msg("No messages found"))?.id)
                .await?;
        } else {
            let message_ids = messages.into_iter().map(|messages| messages.id).collect::<Vec<_>>();
            twilight.delete_messages(channel, &message_ids).await?;
        }

        ctx.respond(|r| r.content("Done!!").ephemeral()).await
    }
}
