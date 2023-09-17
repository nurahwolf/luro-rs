use async_trait::async_trait;
use luro_framework::{
    command::LuroCommandTrait,
    responses::{PunishmentType, Response, StandardResponse},
    Framework, InteractionCommand, LuroInteraction,
};
use luro_model::{database_driver::LuroDatabaseDriver, guild::log_channel::LuroLogChannel};
use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::guild::Permissions;

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "unban", desc = "Unban a user", dm_permission = false)]
pub struct Unban {
    /// The user to ban
    pub user: ResolvedUser,
    /// The reason they should be unbanned.
    pub reason: String,
}

#[async_trait]
impl LuroCommandTrait for Unban {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let interaction = &interaction;
        let guild_id = interaction.guild_id.unwrap();
        let guild = ctx.database.get_guild(&guild_id).await?;
        let luro = ctx
            .database
            .get_user(&ctx.twilight_client.current_user().await?.model().await?.id)
            .await?;
        let mut moderator = interaction.get_interaction_author(&ctx).await?;
        let mut punished_user = ctx.database.get_user(&data.user.resolved.id).await?;
        punished_user.update_user(&data.user.resolved);
        let mut response = interaction.acknowledge_interaction(&ctx, false).await?;
        let moderator_permissions = guild.user_permission(&moderator)?;
        let luro_permissions = guild.user_permission(&luro)?;

        if !luro_permissions.contains(Permissions::BAN_MEMBERS) {
            return Response::BotMissingPermission(Permissions::BAN_MEMBERS)
                .respond(&ctx, interaction)
                .await;
        }

        if !moderator_permissions.contains(Permissions::BAN_MEMBERS) {
            return Response::MissingPermission(Permissions::BAN_MEMBERS)
                .respond(&ctx, interaction)
                .await;
        }

        // Checks passed, now let's action the user
        let mut embed =
            StandardResponse::new_punishment(PunishmentType::Unbanned, &guild.name, &guild_id, &punished_user, &moderator);
        embed.punishment_reason(Some(&data.reason), &punished_user);
        match ctx.twilight_client.create_private_channel(punished_user.id).await {
            Ok(channel) => {
                let victim_dm = ctx
                    .twilight_client
                    .create_message(channel.model().await?.id)
                    .embeds(&[embed.embed().0])
                    .await;

                match victim_dm {
                    Ok(_) => embed.dm_sent(true),
                    Err(_) => embed.dm_sent(false),
                }
            }
            Err(_) => embed.dm_sent(false),
        };

        let unban = ctx
            .twilight_client
            .delete_ban(guild_id, punished_user.id)
            .reason(&data.reason)
            .await;
        match unban {
            Ok(_) => embed.create_field("Unban", "Successful", true),
            Err(_) => embed.create_field("Unban", "Failed", true),
        };

        response.add_embed(embed.embed().0);
        interaction.send_response(&ctx, response).await?;

        moderator.moderation_actions_performed += 1;
        ctx.database.modify_user(&moderator.id, &moderator).await?;

        // If an alert channel is defined, send a message there
        ctx.send_log_channel(&guild_id, LuroLogChannel::Moderator, |r| r.add_embed(embed.embed))
            .await?;

        Ok(())
    }
}
