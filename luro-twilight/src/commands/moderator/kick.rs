use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand, PunishmentType, Response, StandardResponse};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::UserMarker, Id},
};

use super::{reason, Reason};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "kick", desc = "Kick a user", dm_permission = false)]
pub struct Kick {
    /// The user to kick
    pub user: Id<UserMarker>,
    /// The reason they should be kicked.
    pub reason: Reason,
    /// Some added description to why they should be kicked
    pub details: Option<String>,
}

impl LuroCommand for Kick {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let reason = reason(self.reason, self.details);
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

        if !luro_permissions.contains(Permissions::KICK_MEMBERS) {
            return ctx.response_simple(Response::BotMissingPermission(Permissions::KICK_MEMBERS)).await;
        }

        if !moderator_permissions.contains(Permissions::KICK_MEMBERS) {
            return ctx.response_simple(Response::MissingPermission(Permissions::KICK_MEMBERS)).await;
        }

        if guild.is_owner(&target.user_id) {
            return ctx
                .response_simple(Response::PermissionModifyServerOwner(&ctx.author.user_id))
                .await;
        }

        // The lower the number, the higher they are on the heirarchy
        if let Some(punished_user_highest_role) = target_highest_role {
            if let Some(moderator_highest_role) = moderator_highest_role {
                tracing::info!(
                    "Punished user position `{}` | Moderator user position `{}`: `{:#?}`",
                    punished_user_highest_role.position,
                    moderator_highest_role.position,
                    punished_user_highest_role.cmp(moderator_highest_role)
                );
                if punished_user_highest_role <= moderator_highest_role {
                    return ctx.response_simple(Response::UserHeirarchy(&target.name())).await;
                }
            }

            if let Some(luro_highest_role) = luro_highest_role {
                tracing::info!(
                    "Punished user position `{}` | Luro user position `{}`: `{:#?}`",
                    punished_user_highest_role.position,
                    luro_highest_role.position,
                    punished_user_highest_role.cmp(luro_highest_role)
                );
                if punished_user_highest_role <= luro_highest_role {
                    return ctx.response_simple(Response::BotHeirarchy(&luro.name())).await;
                }
            }
        } else {
            tracing::warn!(
                "Could not fetch the highest role for {}! They have no roles in my cache!!",
                target.user_id
            )
        }

        // Checks passed, now let's action the user
        let mut embed = StandardResponse::new_punishment(PunishmentType::Kicked, &guild.name, &guild.guild_id, &target, moderator);
        embed.punishment_reason(reason.as_deref(), &target);
        match ctx.twilight_client.create_private_channel(target.user_id).await {
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

        response.add_embed(embed.embed().0);
        ctx.response_send(response).await?;

        ctx.twilight_client.remove_guild_member(guild.guild_id, target.user_id).await?;

        // moderator.moderation_actions_performed += 1;
        // ctx.database.modify_user(&moderator.id, &moderator).await?;

        // // Record the punishment
        // punished_user.moderation_actions.push(UserActions {
        //     action_type: vec![UserActionType::Kick],
        //     guild_id: Some(guild_id),
        //     reason: reason.clone(),
        //     responsible_user: moderator.id,
        // });
        // ctx.database.modify_user(&punished_user.id, &punished_user).await?;

        // // If an alert channel is defined, send a message there
        // ctx.send_log_channel(&guild_id, LuroLogChannel::Moderator, |r| r.add_embed(embed.embed))
        //     .await?;

        Ok(luro_model::types::CommandResponse::default())
    }
}
