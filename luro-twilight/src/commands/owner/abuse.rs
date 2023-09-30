use async_trait::async_trait;
use luro_builder::embed::EmbedBuilder;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
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
#[async_trait]
impl LuroCommandTrait for Abuse {
    async fn handle_interaction(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let webhook = ctx.get_webhook(interaction.channel.id).await?;
        let webhook_token = match webhook.token {
            Some(token) => token,
            None => match ctx.twilight_client.webhook(webhook.id).await?.model().await?.token {
                Some(token) => token,
                None => {
                    return interaction
                        .respond(&ctx, |r| {
                            r.content("Sorry, I can't setup a webhook here. Probably missing perms.")
                                .ephemeral()
                        })
                        .await
                }
            },
        };

        let luro_user = ctx.database.get_user(&data.user.resolved.id).await?;

        let mut embed = EmbedBuilder::default();
        embed
            .colour(interaction.accent_colour(&ctx).await)
            .description(&data.message)
            .author(|author| author.name(&luro_user.name()).icon_url(&luro_user.avatar()));

        let avatar = luro_user.avatar();
        let name = luro_user.member_name(&interaction.guild_id);
        let webhook_message = ctx
            .twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&name)
            .avatar_url(&avatar);

        match data.embed.unwrap_or_default() {
            true => webhook_message.embeds(&[embed.clone().into()]).await?,
            false => webhook_message.content(&data.message).await?,
        };

        interaction
            .respond(&ctx, |response| response.add_embed(embed).ephemeral())
            .await
    }
}
