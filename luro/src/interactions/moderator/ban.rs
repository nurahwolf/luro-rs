use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    guild::log_channel::LuroLogChannel,
    user::{actions::UserActions, actions_type::UserActionType}
};

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
    pub details: Option<String>
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
    SevenDays
}

impl LuroCommand for Ban {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let interaction = &ctx.interaction;
        let guild_id = interaction.guild_id.unwrap();
        let mut guild = ctx.framework.database.get_guild(&guild_id).await?;
        let luro = ctx
            .framework
            .database
            .get_user(&ctx.framework.twilight_client.current_user().await?.model().await?.id)
            .await?;
        let mut moderator = ctx.get_interaction_author(interaction).await?;
        let mut punished_user = ctx.framework.database.get_user(&self.user.resolved.id).await?;
        punished_user.update_user(&self.user.resolved);
        let mut response = ctx.acknowledge_interaction(false).await?;
        let moderator_permissions = guild.user_permission(&moderator)?;
        let moderator_highest_role = guild.user_highest_role(&moderator);
        let punished_user_highest_role = guild.user_highest_role(&punished_user);
        let luro_permissions = guild.user_permission(&luro)?;
        let luro_highest_role = guild.user_highest_role(&luro);
        let reason = reason(self.reason, self.details);
        let period_string = match self.purge {
            TimeToBan::None => "Don't Delete Any".to_string(),
            TimeToBan::Hour => "Previous Hour".to_string(),
            TimeToBan::SixHours => "Previous 6 Hours".to_string(),
            TimeToBan::TwelveHours => "Previous 12 Hours".to_string(),
            TimeToBan::TwentyFourHours => "Previous 24 Hours".to_string(),
            TimeToBan::ThreeDays => "Previous 3 Days".to_string(),
            TimeToBan::SevenDays => "Previous 7 Days".to_string()
        };

        if !luro_permissions.contains(Permissions::BAN_MEMBERS) {
            return ctx.bot_missing_permission_response(Permissions::BAN_MEMBERS).await;
        }

        if !moderator_permissions.contains(Permissions::BAN_MEMBERS) {
            return ctx.missing_permission_response(Permissions::BAN_MEMBERS).await;
        }

        // Check if the author and the bot have required permissions.
        if guild.is_owner(&punished_user) {
            return ctx.server_owner_response().await;
        }

        if punished_user_highest_role >= moderator_highest_role {
            return ctx.user_hierarchy_response(&punished_user.member_name(&Some(guild_id))).await;
        }

        if punished_user_highest_role >= luro_highest_role {
            let name = ctx.framework.database.current_user.read().unwrap().clone().name;
            return ctx.bot_hierarchy_response(&name).await;
        }

        // Checks passed, now let's action the user
        let mut embed = ctx.framework.ban_embed(
            &guild.name,
            &guild_id,
            &moderator,
            &punished_user,
            reason.as_deref(),
            Some(&period_string)
        );
        match ctx.framework.twilight_client.create_private_channel(punished_user.id).await {
            Ok(channel) => {
                let victim_dm = ctx
                    .framework
                    .twilight_client
                    .create_message(channel.model().await?.id)
                    .embeds(&[embed.clone().into()])
                    .await;

                match victim_dm {
                    Ok(_) => embed.create_field("DM Sent", "Successful", true),
                    Err(_) => embed.create_field("DM Sent", "Failed", true)
                }
            }
            Err(_) => embed.create_field("DM Sent", "Failed", true)
        };

        response.add_embed(embed.clone());
        ctx.send_respond(response).await?;

        let ban = ctx.framework.twilight_client.create_ban(guild_id, punished_user.id);
        match reason {
            None => ban.delete_message_seconds(self.purge as u32).await?,
            Some(ref reason) => ban.delete_message_seconds(self.purge as u32).reason(reason).await?
        };

        moderator.moderation_actions_performed += 1;
        ctx.framework.database.save_user(&moderator.id, &moderator).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Ban],
            guild_id: Some(guild_id),
            reason,
            responsible_user: moderator.id
        });
        ctx.framework.database.save_user(&punished_user.id, &punished_user).await?;

        // If an alert channel is defined, send a message there
        ctx.framework
            .send_log_channel(&Some(guild_id), embed.into(), LuroLogChannel::Moderator)
            .await?;

        Ok(())
    }
}
