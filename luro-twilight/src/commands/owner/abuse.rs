use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::builders::EmbedBuilder;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "abuse", desc = "Use a webhook to pretend to be a user", dm_permission = false)]
pub struct Abuse {
    /// The user to bully
    user: ResolvedUser,
    /// What they should say!
    message: String,
    /// If the message should be sent as an embed
    embed: Option<bool>,
}

impl LuroCommand for Abuse {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let webhook = ctx.get_webhook(ctx.channel.id).await?;
        let webhook_token = match webhook.token {
            Some(token) => token,
            None => match ctx.twilight_client.webhook(webhook.id).await?.model().await?.token {
                Some(token) => token,
                None => {
                    return ctx
                        .respond(|r| {
                            r.content("Sorry, I can't setup a webhook here. Probably missing perms.")
                                .ephemeral()
                        })
                        .await
                }
            },
        };

        let luro_user = ctx.fetch_user(self.user.resolved.id).await?;

        let mut embed = EmbedBuilder::default();
        embed
            .colour(ctx.accent_colour())
            .description(&self.message)
            .author(|author| author.name(&luro_user.name()).icon_url(&luro_user.avatar_url()));

        let avatar = luro_user.avatar_url();
        let name = luro_user.name();
        let webhook_message = ctx
            .twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&name)
            .avatar_url(&avatar);

        match self.embed.unwrap_or_default() {
            true => webhook_message.embeds(&[embed.clone().into()]).await?,
            false => webhook_message.content(&self.message).await?,
        };

        ctx.respond(|response| response.add_embed(embed).ephemeral()).await
    }
}
