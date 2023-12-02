use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::{
    response::{BannedResponse, SimpleResponse},
    types::CommandResponse,
};
use tracing::debug;
use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};
use twilight_model::{
    guild::Permissions,
    id::{marker::UserMarker, Id},
};

use super::{reason, Reason};

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "ban", desc = "Ban a user", dm_permission = false)]
pub struct Ban {
    /// The user to ban
    pub user: Id<UserMarker>,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: TimeToBan,
    /// The reason they should be banned.
    pub reason: Reason,
    /// Some added description to why they should be banned
    pub details: Option<String>,
}

#[derive(CommandOption, CreateOption, Clone, Debug, PartialEq, Eq)]
pub enum TimeToBan {
    #[option(name = "Don't Delete Any", value = 0)]
    None,
    #[option(name = "Previous Hour", value = 3_600)]
    Hour,
    #[option(name = "Previous 6 Hours", value = 21_600)]
    SixHours,
    #[option(name = "Previous 12 Hours", value = 43_200)]
    TwelveHours,
    #[option(name = "Previous 24 Hours", value = 86_400)]
    TwentyFourHours,
    #[option(name = "Previous 3 Days", value = 259_200)]
    ThreeDays,
    #[option(name = "Previous 7 Days", value = 604_800)]
    SevenDays,
}

impl LuroCommand for Ban {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut response = ctx.acknowledge_interaction(false).await?;
        let guild = ctx.guild.as_ref().context("Expected this to be a guild")?;
        let luro = ctx.database.user_fetch_current_user(ctx.guild_id()).await?;
        let moderator = &ctx.author;
        let target = ctx.fetch_user(self.user).await?;
        let luro_data = match &luro.member.as_ref().context("Expected member context")?.data {
            Some(data) => data,
            None => {
                return ctx
                    .respond(|r| {
                        r.content("Sorry, could not fetch my permissions to check if I can do this!")
                            .ephemeral()
                    })
                    .await
            }
        };
        let punished_data = match &target.member.as_ref().context("Expected member context")?.data {
            Some(data) => data,
            None => {
                return ctx
                    .respond(|r| {
                        r.content("Sorry, could not fetch the permissions of who you wish to punish!")
                            .ephemeral()
                    })
                    .await
            }
        };
        let moderator_data = match &moderator.member.as_ref().context("Expected member context")?.data {
            Some(data) => data,
            None => {
                return ctx
                    .respond(|r| {
                        r.content("Sorry, could not fetch your permissions to check if you can do this!")
                            .ephemeral()
                    })
                    .await
            }
        };

        let luro_highest_role = luro_data.highest_role();
        let luro_permissions = luro_data.permission_calculator(&luro_data.role_permissions()).root();
        let moderator_highest_role = moderator_data.highest_role();
        let moderator_permissions = moderator_data.permission_calculator(&moderator_data.role_permissions()).root();
        let target_highest_role = punished_data.highest_role();

        let reason = reason(self.reason, self.details);

        if !luro_permissions.contains(Permissions::BAN_MEMBERS) {
            return ctx
                .simple_response(SimpleResponse::BotMissingPermission(&Permissions::BAN_MEMBERS))
                .await;
        }

        if !moderator_permissions.contains(Permissions::BAN_MEMBERS) {
            return ctx
                .simple_response(SimpleResponse::MissingPermission(&Permissions::BAN_MEMBERS))
                .await;
        }

        if guild.is_owner(&target.user_id) {
            return ctx
                .simple_response(SimpleResponse::PermissionModifyServerOwner(&moderator.user_id))
                .await;
        }

        // The lower the number, the higher they are on the heirarchy
        if let Some(punished_user_highest_role) = target_highest_role {
            tracing::debug!("Punished user position: {}", punished_user_highest_role.position);
            if let Some(moderator_highest_role) = moderator_highest_role {
                tracing::debug!("Moderator user position: {}", moderator_highest_role.position);
                if punished_user_highest_role.position <= moderator_highest_role.position {
                    return ctx.simple_response(SimpleResponse::UserHeirarchy(&target.name())).await;
                }
            }

            if let Some(luro_highest_role) = luro_highest_role {
                tracing::debug!("Luro user position: {}", luro_highest_role.position);
                if punished_user_highest_role.position <= luro_highest_role.position {
                    return ctx.simple_response(SimpleResponse::BotHeirarchy(&luro.name())).await;
                }
            }
        } else {
            tracing::warn!(
                "Could not fetch the highest role for {}! They have no roles in my cache!!",
                target.user_id
            )
        }

        // Checks passed, now let's action the user

        let embed = match ctx.twilight_client.create_private_channel(target.user_id).await {
            Ok(channel) => {
                let victim_dm = ctx
                    .twilight_client
                    .create_message(channel.model().await?.id)
                    .embeds(&[SimpleResponse::BannedUserResponse(
                        BannedResponse {
                            target: &target,
                            moderator,
                            reason: reason.as_deref(),
                            purged_messages: self.purge.value(),
                        },
                        &guild.name,
                    )
                    .embed()])
                    .await;

                match victim_dm {
                    Ok(_) => SimpleResponse::BannedModeratorResponse(
                        BannedResponse {
                            target: &target,
                            moderator,
                            reason: reason.as_deref(),
                            purged_messages: self.purge.value(),
                        },
                        true,
                    )
                    .embed(),
                    Err(_) => SimpleResponse::BannedModeratorResponse(
                        BannedResponse {
                            target: &target,
                            moderator,
                            reason: reason.as_deref(),
                            purged_messages: self.purge.value(),
                        },
                        false,
                    )
                    .embed(),
                }
            }
            Err(_) => SimpleResponse::BannedModeratorResponse(
                BannedResponse {
                    target: &target,
                    moderator,
                    reason: reason.as_deref(),
                    purged_messages: self.purge.value(),
                },
                false,
            )
            .embed(),
        };

        response.add_embed(embed);
        ctx.response_send(response).await?;

        let ban = ctx.twilight_client.create_ban(guild.guild_id, target.user_id);
        debug!("Purging {:#?} seconds worth of messages!", self.purge.value());

        match reason {
            None => ban.delete_message_seconds(self.purge.value() as u32).await?,
            Some(ref reason) => ban.delete_message_seconds(self.purge as u32).reason(reason).await?,
        };

        // moderator.moderation_actions_performed += 1;
        // ctx.database.modify_user(&moderator.id, &moderator).await?;

        // // Record the punishment
        // punished_user.moderation_actions.push(UserActions {
        //     action_type: vec![UserActionType::Ban],
        //     guild_id: Some(guild_id),
        //     reason,
        //     responsible_user: moderator.id,
        // });
        // ctx.database.modify_user(&punished_user.id, &punished_user).await?;

        // If an alert channel is defined, send a message there
        // ctx.send_log_channel(LuroLogChannel::Moderator, |r| r.add_embed(embed.embed))
        //     .await?;

        Ok(CommandResponse::default())
    }
}
