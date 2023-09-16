use async_trait::async_trait;
use luro_framework::command::LuroCommandTrait;
use luro_framework::responses::{PunishmentType, SimpleResponse, StandardResponse};
use luro_framework::{Framework, InteractionCommand, LuroInteraction};

use luro_model::database_driver::LuroDatabaseDriver;
use luro_model::{
    guild::log_channel::LuroLogChannel,
    user::{actions::UserActions, actions_type::UserActionType},
};
use tracing::{info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::guild::Permissions;

use super::{reason, Reason};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
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
    pub details: Option<String>,
}

#[async_trait]
impl LuroCommandTrait for Kick {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let interaction = interaction;
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
        let moderator_highest_role = guild.user_highest_role(&moderator);
        let punished_user_highest_role = guild.user_highest_role(&punished_user);
        let luro_permissions = guild.user_permission(&luro)?;
        let luro_highest_role = guild.user_highest_role(&luro);
        let reason = reason(data.reason, data.details);

        if !luro_permissions.contains(Permissions::KICK_MEMBERS) {
            return SimpleResponse::BotMissingPermission(Permissions::KICK_MEMBERS)
                .respond(&ctx, &interaction)
                .await;
        }

        if !moderator_permissions.contains(Permissions::KICK_MEMBERS) {
            return SimpleResponse::MissingPermission(Permissions::KICK_MEMBERS)
                .respond(&ctx, &interaction)
                .await;
        }

        // Check if the author and the bot have required permissions.
        if guild.is_owner(&punished_user) {
            return SimpleResponse::PermissionModifyServerOwner(&moderator.id)
                .respond(&ctx, &interaction)
                .await;
        }

        // The lower the number, the higher they are on the heirarchy
        if let Some(punished_user_highest_role) = punished_user_highest_role {
            info!("Punished user position: {}", punished_user_highest_role.0);
            if let Some(moderator_highest_role) = moderator_highest_role {
                info!("Moderator user position: {}", moderator_highest_role.0);
                if punished_user_highest_role.0 <= moderator_highest_role.0 {
                    return SimpleResponse::UserHeirarchy(&punished_user.member_name(&Some(guild_id)))
                        .respond(&ctx, &interaction)
                        .await;
                }
            }

            if let Some(luro_highest_role) = luro_highest_role {
                info!("Luro user position: {}", luro_highest_role.0);
                if punished_user_highest_role.0 <= luro_highest_role.0 {
                    let name = ctx.database.current_user.read().unwrap().clone().name;
                    return SimpleResponse::BotHeirarchy(&name).respond(&ctx, &interaction).await;
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
            StandardResponse::new_punishment(PunishmentType::Kicked, &guild.name, &guild.id, &punished_user, &moderator);
        embed.punishment_reason(reason.as_deref(), &punished_user);
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

        response.add_embed(embed.embed().0);
        interaction.send_response(&ctx, response).await?;

        ctx.twilight_client.remove_guild_member(guild_id, punished_user.id).await?;

        moderator.moderation_actions_performed += 1;
        ctx.database.modify_user(&moderator.id, &moderator).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Kick],
            guild_id: Some(guild_id),
            reason: reason.clone(),
            responsible_user: moderator.id,
        });
        ctx.database.modify_user(&punished_user.id, &punished_user).await?;

        // If an alert channel is defined, send a message there
        ctx.send_log_channel(&guild_id, LuroLogChannel::Moderator, |r| r.add_embed(embed.embed))
            .await?;

        Ok(())
    }
}
