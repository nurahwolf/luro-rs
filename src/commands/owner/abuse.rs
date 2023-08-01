use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_util::builder::embed::{EmbedAuthorBuilder, ImageSource};

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
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
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

        let user_avatar = self.get_user_avatar(&self.user.resolved);
        let username = match self.user.member {
            Some(member) => member.nick.unwrap_or(self.user.resolved.name),
            None => self.user.resolved.name
        };
        let embed = ctx
            .default_embed()
            .await?
            .description(&self.message)
            .author(EmbedAuthorBuilder::new(username.clone()).icon_url(ImageSource::url(&user_avatar)?))
            .build();

        let webhook_message = ctx
            .luro
            .twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&username)?
            .avatar_url(&user_avatar);

        if let Some(embed_wanted) = self.embed && embed_wanted {
            webhook_message.embeds(&[embed.clone()])?.await?

        } else {
            webhook_message.content(&self.message)?.await?
        };

        ctx.embed(embed)?.ephemeral().respond().await
    }
}
