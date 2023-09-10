use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_framework::responses::{PunishmentType, StandardResponse};
use luro_model::{database::drivers::LuroDatabaseDriver, guild::log_channel::LuroLogChannel};

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

impl LuroCommand for Unban {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let interaction = &ctx.interaction;
        let guild_id = interaction.guild_id.unwrap();
        let guild = ctx.framework.database.get_guild(&guild_id).await?;
        let luro = ctx
            .framework
            .database
            .get_user(&ctx.framework.twilight_client.current_user().await?.model().await?.id, false)
            .await?;
        let mut moderator = ctx.get_interaction_author(interaction).await?;
        let mut punished_user = ctx.framework.database.get_user(&self.user.resolved.id, false).await?;
        punished_user.update_user(&self.user.resolved);
        let mut response = ctx.acknowledge_interaction(false).await?;
        let moderator_permissions = guild.user_permission(&moderator)?;
        let luro_permissions = guild.user_permission(&luro)?;

        if !luro_permissions.contains(Permissions::BAN_MEMBERS) {
            return ctx.bot_missing_permission_response(Permissions::BAN_MEMBERS).await;
        }

        if !moderator_permissions.contains(Permissions::BAN_MEMBERS) {
            return ctx.missing_permission_response(Permissions::BAN_MEMBERS).await;
        }

        // Checks passed, now let's action the user
        let mut embed =
            StandardResponse::new_punishment(PunishmentType::Unbanned, &guild.name, &guild_id, &punished_user, &moderator);
        embed.punishment_reason(Some(&self.reason), &punished_user);
        match ctx.framework.twilight_client.create_private_channel(punished_user.id).await {
            Ok(channel) => {
                let victim_dm = ctx
                    .framework
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
            .framework
            .twilight_client
            .delete_ban(guild_id, punished_user.id)
            .reason(&self.reason)
            .await;
        match unban {
            Ok(_) => embed.create_field("Unban", "Successful", true),
            Err(_) => embed.create_field("Unban", "Failed", true),
        };

        response.add_embed(embed.embed().0);
        ctx.send_respond(response).await?;

        moderator.moderation_actions_performed += 1;
        ctx.framework.database.save_user(&moderator.id, &moderator).await?;

        // If an alert channel is defined, send a message there
        ctx.framework
            .send_log_channel(&Some(guild_id), embed.embed.0, LuroLogChannel::Moderator)
            .await?;

        Ok(())
    }
}
