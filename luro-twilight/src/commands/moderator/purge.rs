use anyhow::Error;

use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "purge",
    desc = "Remove up to 100 messages from a channel",
    default_permissions = "Self::default_permissions"
)]
pub struct Purge {
    /// Choose how many messages should be removed
    #[command(min_value = 1, max_value = 100)]
    amount: i64,
}

#[async_trait]
impl LuroCommandTrait for Purge {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let channel = interaction.channel.id;
        let twilight = &ctx.twilight_client;
        let messages = twilight
            .channel_messages(channel)
            .limit(data.amount as u16)
            .await?
            .model()
            .await?;

        if data.amount == 1 {
            twilight
                .delete_message(channel, messages.first().ok_or_else(|| Error::msg("No messages found"))?.id)
                .await?;
        } else {
            let message_ids = messages.into_iter().map(|messages| messages.id).collect::<Vec<_>>();
            twilight.delete_messages(channel, &message_ids).await?;
        }

        interaction.respond(&ctx, |r| r.content("Done!!").ephemeral()).await
    }
}
