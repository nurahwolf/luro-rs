use anyhow::Context;

use luro_builder::embed::EmbedBuilder;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
use crate::models::LuroWebhook;

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
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let luro_webhook = LuroWebhook::new(ctx.framework.clone());
        let webhook = luro_webhook
            .get_webhook(
                ctx.interaction
                    .channel
                    .clone()
                    .context("Could not get channel the interaction is in")?
                    .id
            )
            .await?;
        let webhook_token = match webhook.token {
            Some(token) => token,
            None => match ctx.framework.twilight_client.webhook(webhook.id).await?.model().await?.token {
                Some(token) => token,
                None => {
                    return ctx
                        .respond(|r| {
                            r.content("Sorry, I can't setup a webhook here. Probably missing perms.")
                                .ephemeral()
                        })
                        .await
                }
            }
        };

        let luro_user = ctx.framework.database.get_user(&self.user.resolved.id).await?;

        let mut embed = EmbedBuilder::default();
        embed
            .colour(ctx.accent_colour().await)
            .description(&self.message)
            .author(|author| author.name(&luro_user.name()).icon_url(&luro_user.avatar()));

        let avatar = luro_user.avatar();
        let webhook_message = ctx
            .framework
            .twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&luro_user.name)
            .avatar_url(&avatar);

        match self.embed.unwrap_or_default() {
            true => webhook_message.embeds(&[embed.clone().into()]).await?,
            false => webhook_message.content(&self.message).await?
        };

        ctx.respond(|response| response.add_embed(embed)).await
    }
}
