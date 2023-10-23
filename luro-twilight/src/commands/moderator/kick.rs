use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand, PunishmentType, Response, StandardResponse};
use tracing::{debug, warn};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::guild::Permissions;

use super::{reason, Reason};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "kick", desc = "Kick a user", dm_permission = false)]
pub struct Kick {
    /// The user to kick
    pub user: ResolvedUser,
    /// The reason they should be kicked.
    pub reason: Reason,
    /// Some added description to why they should be kicked
    pub details: Option<String>,
}

impl LuroCommand for Kick {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let guild = ctx.guild.as_ref().context("Expected this to be a guild")?;
        let luro = ctx
            .fetch_user(&ctx.twilight_client.current_user().await?.model().await?.id, true)
            .await?;

        let punished_user = ctx.fetch_user(&self.user.resolved.id, true).await?;
        let mut response = ctx.acknowledge_interaction(false).await?;
        let moderator_permissions = ctx
            .author
            .member
            .as_ref()
            .context("Expected member context")?
            .permission_calculator(
                ctx.database(),
                &ctx.author.member.as_ref().context("Expected member context")?.role_permissions(),
            )
            .await?
            .root();
        let moderator_highest_role = guild.get_member_highest_role(&ctx.author.member);
        let punished_user_highest_role = guild.get_member_highest_role(&punished_user.member);
        let luro_permissions = luro
            .member
            .as_ref()
            .context("Expected member context")?
            .permission_calculator(
                ctx.database(),
                &luro.member.as_ref().context("Expected member context")?.role_permissions(),
            )
            .await?
            .root();
        let luro_highest_role = guild.get_member_highest_role(&luro.member);
        let reason = reason(self.reason, self.details);

        if !luro_permissions.contains(Permissions::KICK_MEMBERS) {
            return ctx.response_simple(Response::BotMissingPermission(Permissions::KICK_MEMBERS)).await;
        }

        if !moderator_permissions.contains(Permissions::KICK_MEMBERS) {
            return ctx.response_simple(Response::MissingPermission(Permissions::KICK_MEMBERS)).await;
        }

        if !guild.is_owner(&punished_user.user_id()) {
            return ctx
                .response_simple(Response::PermissionModifyServerOwner(&ctx.author.user_id()))
                .await;
        }

        // The lower the number, the higher they are on the heirarchy
        if let Some(punished_user_highest_role) = punished_user_highest_role {
            debug!("Punished user position: {}", punished_user_highest_role.position);
            if let Some(moderator_highest_role) = moderator_highest_role {
                debug!("Moderator user position: {}", moderator_highest_role.position);
                if punished_user_highest_role.position <= moderator_highest_role.position {
                    return ctx.response_simple(Response::UserHeirarchy(&punished_user.name)).await;
                }
            }

            if let Some(luro_highest_role) = luro_highest_role {
                debug!("Luro user position: {}", luro_highest_role.position);
                if punished_user_highest_role.position <= luro_highest_role.position {
                    return ctx.response_simple(Response::BotHeirarchy(&luro.name())).await;
                }
            }
        } else {
            warn!(
                "Could not fetch the highest role for {}! They have no roles in my cache!!",
                punished_user.user_id
            )
        }

        // Checks passed, now let's action the user
        let mut embed =
            StandardResponse::new_punishment(PunishmentType::Kicked, &guild.name, &guild.guild_id(), &punished_user, &ctx.author);
        embed.punishment_reason(reason.as_deref(), &punished_user);
        match ctx.twilight_client.create_private_channel(punished_user.user_id()).await {
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

        ctx.twilight_client
            .remove_guild_member(guild.guild_id(), punished_user.user_id())
            .await?;

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

        Ok(())
    }
}
