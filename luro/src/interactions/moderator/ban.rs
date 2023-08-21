use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    guild::log_channel::LuroLogChannel,
    legacy::guild_permissions::GuildPermissions,
    user::{actions::UserActions, actions_type::UserActionType}
};
use std::convert::TryInto;

use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption, ResolvedUser};
use twilight_model::{guild::Permissions, id::Id};

use crate::{interaction::LuroSlash, interactions::moderator::reason, luro_command::LuroCommand};

use super::Reason;

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
        let author_user_id = interaction.author_id().unwrap();
        let mut author_user = ctx.framework.database.get_user(&author_user_id).await?;
        let mut punished_user = ctx.framework.database.get_user(&self.user.resolved.id).await?;
        let mut response = ctx.acknowledge_interaction(false).await?;

        let guild_id = interaction.guild_id.unwrap();
        let permissions = GuildPermissions::new(&ctx.framework.twilight_client, &guild_id).await?;
        let author_member = interaction.member.as_ref().unwrap();
        let author_permissions = permissions.member(author_user_id, &author_member.roles).await?;
        let bot_permissions = permissions.current_member().await?;
        let guild = ctx.framework.twilight_client.guild(guild_id).await?.model().await?;
        let punished_user_id = Id::new(punished_user.id);
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

        // Permission checks
        if reason.is_empty() {
            response.content("You need to specify a reason, dork!").ephemeral();
            return ctx.send_respond(response).await;
        }

        if !bot_permissions.guild().contains(Permissions::BAN_MEMBERS) {
            return ctx.bot_missing_permission_response(&"BAN_MEMBERS".to_owned()).await;
        }

        // Checks if we have them recorded as a member of the guild
        if let Some(punished_member) = self.user.member {
            // The user is a member of the server, so carry out some additional checks.
            let member_permissions = permissions.member(punished_user_id, &punished_member.roles).await?;
            let member_highest_role = member_permissions.highest_role();

            // Check if the author and the bot have required permissions.
            if member_permissions.is_owner() {
                return ctx.server_owner_response().await;
            }

            if member_highest_role >= author_permissions.highest_role() {
                return ctx.user_hierarchy_response(&punished_user.member_name(&Some(guild_id))).await;
            }

            if member_highest_role >= bot_permissions.highest_role() {
                let name = ctx.framework.database.current_user.read().unwrap().clone().name;
                return ctx.bot_hierarchy_response(&name).await;
            }
        };

        // Checks passed, now let's action the user
        let mut embed = ctx.ban_embed(&guild, &punished_user, &reason, &period_string).await?;
        let punished_user_dm = match ctx.framework.twilight_client.create_private_channel(punished_user_id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => return ctx.ban_response(&guild, &punished_user, &reason, &period_string, false).await
        };

        let victim_dm = ctx
            .framework
            .twilight_client
            .create_message(punished_user_dm.id)
            .embeds(&[embed.clone().into()])
            .await;

        match victim_dm {
            Ok(_) => embed.create_field("DM Sent", "Successful", true),
            Err(_) => embed.create_field("DM Sent", "Failed", true)
        };

        response.add_embed(embed.clone());
        ctx.send_respond(response).await?;

        let ban = ctx.framework.twilight_client.create_ban(guild_id, punished_user_id);
        match reason.is_empty() {
            true => ban.delete_message_seconds(self.purge as u32).await?,
            false => ban.delete_message_seconds(self.purge as u32).reason(&reason).await?
        };

        author_user.moderation_actions_performed += 1;
        ctx.framework.database.save_user(&author_user_id, &author_user).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Ban],
            guild_id: Some(guild_id),
            reason,
            responsible_user: author_user_id
        });
        ctx.framework.database.save_user(&punished_user_id, &punished_user).await?;

        // If an alert channel is defined, send a message there
        ctx.framework
            .send_log_channel(&Some(guild_id), embed.into(), LuroLogChannel::Moderator)
            .await?;

        Ok(())
    }
}
