use async_trait::async_trait;
use luro_framework::{
    command::LuroCommandTrait,
    responses::{PunishmentType, SimpleResponse, StandardResponse},
    Framework, InteractionCommand, LuroInteraction,
};
use luro_model::{
    database_driver::LuroDatabaseDriver,
    guild::log_channel::LuroLogChannel,
    user::{actions::UserActions, actions_type::UserActionType},
};

use tracing::{debug, warn};
use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption, ResolvedUser};
use twilight_model::guild::Permissions;

use super::{reason, Reason};

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "ban", desc = "Ban a user", dm_permission = false)]
pub struct Ban {
    /// The user to ban
    pub user: ResolvedUser,
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

#[async_trait]
impl LuroCommandTrait for Ban {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
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
        let mut response = interaction.acknowledge_interaction(&ctx, false).await?;
        let moderator_permissions = guild.user_permission(&moderator)?;
        let moderator_highest_role = guild.user_highest_role(&moderator);
        let punished_user_highest_role = guild.user_highest_role(&punished_user);
        let luro_permissions = guild.user_permission(&luro)?;
        let luro_highest_role = guild.user_highest_role(&luro);
        let reason = reason(data.reason, data.details);
        let period_string = match data.purge {
            TimeToBan::None => "Don't Delete Any".to_string(),
            TimeToBan::Hour => "Previous Hour".to_string(),
            TimeToBan::SixHours => "Previous 6 Hours".to_string(),
            TimeToBan::TwelveHours => "Previous 12 Hours".to_string(),
            TimeToBan::TwentyFourHours => "Previous 24 Hours".to_string(),
            TimeToBan::ThreeDays => "Previous 3 Days".to_string(),
            TimeToBan::SevenDays => "Previous 7 Days".to_string(),
        };

        if !luro_permissions.contains(Permissions::BAN_MEMBERS) {
            return SimpleResponse::BotMissingPermission(Permissions::BAN_MEMBERS)
                .respond(&ctx, interaction)
                .await;
        }

        if !moderator_permissions.contains(Permissions::BAN_MEMBERS) {
            return SimpleResponse::BotMissingPermission(Permissions::BAN_MEMBERS)
                .respond(&ctx, interaction)
                .await;
        }

        // Check if the author and the bot have required permissions.
        if guild.is_owner(&punished_user) {
            return SimpleResponse::PermissionModifyServerOwner(&moderator.id)
                .respond(&ctx, interaction)
                .await;
        }

        // The lower the number, the higher they are on the heirarchy
        if let Some(punished_user_highest_role) = punished_user_highest_role {
            debug!("Punished user position: {}", punished_user_highest_role.0);
            if let Some(moderator_highest_role) = moderator_highest_role {
                debug!("Moderator user position: {}", moderator_highest_role.0);
                if punished_user_highest_role.0 <= moderator_highest_role.0 {
                    return SimpleResponse::UserHeirarchy(&punished_user.member_name(&Some(guild_id)))
                        .respond(&ctx, interaction)
                        .await;
                }
            }

            if let Some(luro_highest_role) = luro_highest_role {
                debug!("Luro user position: {}", luro_highest_role.0);
                if punished_user_highest_role.0 <= luro_highest_role.0 {
                    let name = ctx.database.current_user.read().unwrap().clone().name;
                    return SimpleResponse::BotHeirarchy(&name).respond(&ctx, interaction).await;
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
            StandardResponse::new_punishment(PunishmentType::Banned, &guild.name, &guild.id, &punished_user, &moderator);
        embed
            .punishment_reason(reason.as_deref(), &punished_user)
            .punishment_period(&period_string);
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

        let ban = ctx.twilight_client.create_ban(guild_id, punished_user.id);
        debug!("Purging {:#?} seconds worth of messages!", data.purge.value());

        match reason {
            None => ban.delete_message_seconds(data.purge.value() as u32).await?,
            Some(ref reason) => ban.delete_message_seconds(data.purge as u32).reason(reason).await?,
        };

        moderator.moderation_actions_performed += 1;
        ctx.database.modify_user(&moderator.id, &moderator).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Ban],
            guild_id: Some(guild_id),
            reason,
            responsible_user: moderator.id,
        });
        ctx.database.modify_user(&punished_user.id, &punished_user).await?;

        // If an alert channel is defined, send a message there
        ctx.send_log_channel(&guild_id, LuroLogChannel::Moderator, |r| r.add_embed(embed.embed))
            .await?;

        Ok(())
    }
}
