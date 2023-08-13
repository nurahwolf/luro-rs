use anyhow::Context;

use luro_builder::embed::EmbedBuilder;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::models::{LuroWebhook, SlashUser};

use crate::interaction::LuroSlash;

use crate::luro_command::LuroCommand;

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

impl LuroCommand for AbuseCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
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

        let mut embed = EmbedBuilder::default();
        embed
            .colour(ctx.accent_colour().await)
            .description(&self.message)
            .author(|author| author.name(&slash_author.name).icon_url(&slash_author.avatar));

        let webhook_message = ctx
            .framework
            .twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&slash_author.name)
            .avatar_url(&slash_author.avatar);

        match self.embed.unwrap_or_default() {
            true => webhook_message.embeds(&[embed.clone().into()]).await?,
            false => webhook_message.content(&self.message).await?
        };

        ctx.respond(|response| response.add_embed(embed)).await
    }
}
