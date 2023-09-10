use crate::interactions::send;
use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_framework::responses::permission_modify_server_owner::permission_server_owner;
use luro_framework::responses::{PunishmentType, StandardResponse};
use luro_model::database::drivers::LuroDatabaseDriver;

use luro_model::{
    guild::log_channel::LuroLogChannel,
    user::{actions::UserActions, actions_type::UserActionType},
};
use tracing::{info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::guild::Permissions;

use super::{reason, Reason};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq,)]
#[command(
    name = "kick",
    desc = "Kick a user",
    dm_permission = false,
    default_permissions = "Self::default_permissions"
)]
pub struct Kick {
    /// The user to kick
    pub user: ResolvedUser,
    /// The reason they should be kicked.
    pub reason: Reason,
    /// Some added description to why they should be kicked
    pub details: Option<String,>,
}

impl LuroCommand for Kick {
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        let interaction = &ctx.interaction;
        let guild_id = interaction.guild_id.unwrap();
        let guild = ctx.framework.database.get_guild(&guild_id,).await?;
        let luro = ctx
            .framework
            .database
            .get_user(&ctx.framework.twilight_client.current_user().await?.model().await?.id, false,)
            .await?;
        let mut moderator = ctx.get_interaction_author(interaction,).await?;
        let mut punished_user = ctx.framework.database.get_user(&self.user.resolved.id, false,).await?;
        punished_user.update_user(&self.user.resolved,);
        let mut response = ctx.acknowledge_interaction(false,).await?;
        let moderator_permissions = guild.user_permission(&moderator,)?;
        let moderator_highest_role = guild.user_highest_role(&moderator,);
        let punished_user_highest_role = guild.user_highest_role(&punished_user,);
        let luro_permissions = guild.user_permission(&luro,)?;
        let luro_highest_role = guild.user_highest_role(&luro,);
        let reason = reason(self.reason, self.details,);

        if !luro_permissions.contains(Permissions::KICK_MEMBERS,) {
            return ctx.bot_missing_permission_response(Permissions::KICK_MEMBERS,).await;
        }

        if !moderator_permissions.contains(Permissions::KICK_MEMBERS,) {
            return ctx.missing_permission_response(Permissions::KICK_MEMBERS,).await;
        }

        // Check if the author and the bot have required permissions.
        if guild.is_owner(&punished_user,) {
            return send(response.set_embed(permission_server_owner(&moderator.id,),), ctx,).await;
        }

        // The lower the number, the higher they are on the heirarchy
        if let Some(punished_user_highest_role,) = punished_user_highest_role {
            info!("Punished user position: {}", punished_user_highest_role.0);
            if let Some(moderator_highest_role,) = moderator_highest_role {
                info!("Moderator user position: {}", moderator_highest_role.0);
                if punished_user_highest_role.0 <= moderator_highest_role.0 {
                    return ctx
                        .user_hierarchy_response(&punished_user.member_name(&Some(guild_id,),),)
                        .await;
                }
            }

            if let Some(luro_highest_role,) = luro_highest_role {
                info!("Luro user position: {}", luro_highest_role.0);
                if punished_user_highest_role.0 <= luro_highest_role.0 {
                    let name = ctx.framework.database.current_user.read().unwrap().clone().name;
                    return ctx.bot_hierarchy_response(&name,).await;
                }
            }
        } else {
            warn!(
                "Could not fetch the highest role for {}! They have no roles in my cache!!",
                punished_user.id
            )
        }

        // Checks passed, now let's action the user
        let mut embed =
            StandardResponse::new_punishment(PunishmentType::Kicked, &guild.name, &guild.id, &punished_user, &moderator,);
        embed.punishment_reason(reason.as_deref(), &punished_user,);
        match ctx.framework.twilight_client.create_private_channel(punished_user.id,).await {
            Ok(channel,) => {
                let victim_dm = ctx
                    .framework
                    .twilight_client
                    .create_message(channel.model().await?.id,)
                    .embeds(&[embed.embed().0,],)
                    .await;

                match victim_dm {
                    Ok(_,) => embed.dm_sent(true,),
                    Err(_,) => embed.dm_sent(false,),
                }
            }
            Err(_,) => embed.dm_sent(false,),
        };

        response.add_embed(embed.embed().0,);
        ctx.send_respond(response,).await?;

        ctx.framework
            .twilight_client
            .remove_guild_member(guild_id, punished_user.id,)
            .await?;

        moderator.moderation_actions_performed += 1;
        ctx.framework.database.save_user(&moderator.id, &moderator,).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Kick],
            guild_id: Some(guild_id,),
            reason: reason.clone(),
            responsible_user: moderator.id,
        },);
        ctx.framework.database.save_user(&punished_user.id, &punished_user,).await?;

        // If an alert channel is defined, send a message there
        ctx.framework
            .send_log_channel(&Some(guild_id,), embed.embed.0, LuroLogChannel::Moderator,)
            .await?;

        Ok((),)
    }
}
