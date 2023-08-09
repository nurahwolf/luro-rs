use std::convert::TryInto;

use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_util::builder::embed::EmbedAuthorBuilder;

use crate::models::SlashUser;

use crate::models::LuroWebhook;
use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "abuse", desc = "Use a webhook to pretend to be a user", dm_permission = false)]
pub struct AbuseCommand {
    /// The user to bully
    user: ResolvedUser,
    /// What they should say!
    message: String,
    /// If the message should be sent as an embed
    embed: Option<bool>
}

#[async_trait]
impl LuroCommand for AbuseCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let luro_webhook = LuroWebhook::new(ctx.framework.clone()).await?;
        let webhook = luro_webhook
            .get_webhook(
                ctx.interaction
                    .channel
                    .clone()
                    .context("Could not get channel the interaction is in")?
                    .id
            )
            .await?;
        let webhook_token = webhook.token.context("Expected webhook token")?;

        let (_, slash_author) = SlashUser::client_fetch_user(&ctx.framework, self.user.resolved.id).await?;

        let embed = ctx
            .default_embed()
            .await?
            .description(&self.message)
            .author(EmbedAuthorBuilder::new(&slash_author.name).icon_url(slash_author.clone().try_into()?))
            .build();

        let webhook_message = ctx
            .framework
            .twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&slash_author.name)
            .avatar_url(&slash_author.avatar);

        if let Some(embed_wanted) = self.embed && embed_wanted {
            webhook_message.embeds(&[embed.clone()]).await?

        } else {
            webhook_message.content(&self.message).await?
        };

        ctx.embed(embed)?.ephemeral().respond().await
    }
}
