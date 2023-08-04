use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_util::builder::embed::{EmbedAuthorBuilder, ImageSource};

use crate::traits::luro_functions::LuroFunctions;
use crate::{models::LuroSlash, models::LuroWebhook};

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
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let luro_webhook = LuroWebhook::new(ctx.luro.clone()).await?;
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

        let (_user, avatar, name) = ctx.get_specified_user(&self.user, &ctx.interaction);

        let embed = ctx
            .default_embed()
            .await?
            .description(&self.message)
            .author(EmbedAuthorBuilder::new(name.clone()).icon_url(ImageSource::url(&avatar)?))
            .build();

        let webhook_message = ctx
            .luro
            .twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(name)?
            .avatar_url(&avatar);

        if let Some(embed_wanted) = self.embed && embed_wanted {
            webhook_message.embeds(&[embed.clone()])?.await?

        } else {
            webhook_message.content(&self.message)?.await?
        };

        ctx.embed(embed)?.ephemeral().respond().await
    }
}
