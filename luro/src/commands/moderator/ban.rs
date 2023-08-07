use crate::{traits::toml::LuroTOML, USERDATA_FILE_PATH};
use std::{convert::TryInto, path::Path};

use anyhow::Context;
use async_trait::async_trait;
use luro_model::{user_actions::UserActions, user_actions_type::UserActionType};
use tracing::debug;

use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption, ResolvedUser};
use twilight_model::guild::Permissions;
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    models::{GuildPermissions, LuroSlash, UserData},
    traits::luro_command::LuroCommand
};

use super::Reason;

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "ban", desc = "Ban a user", dm_permission = false)]
pub struct BanCommand {
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

#[async_trait]
impl LuroCommand for BanCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.deferred().await?;

        let mut reason = match self.reason {
            Reason::ArtScam => "[Art Scam]".to_owned(),
            Reason::Compromised => "[Compromised Account]".to_owned(),
            Reason::Custom => String::new(),
            Reason::Raider => "[Raider]".to_owned(),
            Reason::Troll => "[Troll]".to_owned(),
            Reason::Vile => "[Vile]".to_owned()
        };

        if let Some(details) = self.details {
            reason.push_str(&format!(" - {details}"))
        }

        if reason.is_empty() {
            return ctx.content("You need to specify a reason, dork!").ephemeral().respond().await;
        }

        let period_string = match self.purge {
            TimeToBan::None => "Don't Delete Any".to_string(),
            TimeToBan::Hour => "Previous Hour".to_string(),
            TimeToBan::SixHours => "Previous 6 Hours".to_string(),
            TimeToBan::TwelveHours => "Previous 12 Hours".to_string(),
            TimeToBan::TwentyFourHours => "Previous 24 Hours".to_string(),
            TimeToBan::ThreeDays => "Previous 3 Days".to_string(),
            TimeToBan::SevenDays => "Previous 7 Days".to_string()
        };
        // Fetch the author and the bot permissions.
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await
        };
        let guild = ctx.luro.twilight_client.guild(guild_id).await?.model().await?;
        let author_user = match &ctx.interaction.member {
            Some(member) => match &member.user {
                Some(user) => user,
                None => return ctx.not_guild_response().await
            },
            None => return ctx.not_guild_response().await
        };
        let author = ctx
            .luro
            .twilight_client
            .guild_member(guild_id, author_user.id)
            .await?
            .model()
            .await?;
        debug!("Getting permissions of the guild");
        let permissions = GuildPermissions::new(&ctx.luro.twilight_client, &guild_id).await?;
        debug!("Getting author permissions");
        let author_permissions = permissions.member(author.user.id, &author.roles).await?;
        let user_to_remove = self.user.resolved;
        debug!("Getting bot permissions");
        let bot_permissions = permissions.current_member().await?;

        if !bot_permissions.guild().contains(Permissions::BAN_MEMBERS) {
            return ctx.bot_missing_permission_response(&"BAN_MEMBERS".to_owned()).await;
        }

        if let Some(member_to_remove) = self.user.member {
            // The user is a member of the server, so carry out some additional checks.
            debug!("Getting banned user's permissions");
            let member_permissions = permissions.member(user_to_remove.id, &member_to_remove.roles).await?;

            // Check if the author and the bot have required permissions.
            if member_permissions.is_owner() {
                return ctx.server_owner_response().await;
            }

            // Check if the role hierarchy allow the author and the bot to perform
            // the ban.
            let member_highest_role = member_permissions.highest_role();

            if member_highest_role >= author_permissions.highest_role() {
                return ctx
                    .user_hierarchy_response(&member_to_remove.nick.unwrap_or(user_to_remove.name.to_owned()))
                    .await;
            }

            if member_highest_role >= bot_permissions.highest_role() {
                let global_data = ctx.luro.global_data.read().clone();
                return ctx.bot_hierarchy_response(&global_data.current_user.name).await;
            }
        };

        // Checks passed, now let's action the user
        let user_to_ban_dm = match ctx.luro.twilight_client.create_private_channel(user_to_remove.id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => {
                return ctx
                    .ban_response(guild, author.clone(), user_to_remove, &reason, &period_string, false)
                    .await
            }
        };

        let mut embed = ctx
            .ban_embed(guild.clone(), author.clone(), user_to_remove.clone(), &reason, &period_string)
            .await?;

        let victim_dm = ctx
            .luro
            .twilight_client
            .create_message(user_to_ban_dm.id)
            .embeds(&[embed.clone().build()])
            .await;

        match victim_dm {
            Ok(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline()),
            Err(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        }

        let mut ban = ctx
            .luro
            .twilight_client
            .create_ban(guild_id, user_to_remove.id)
            .delete_message_seconds(self.purge.value().try_into()?);
        if !reason.is_empty() {
            ban = ban.reason(&reason)
        }
        ban.await?;

        {
            let mut reward = UserData::modify_user_settings(&ctx.luro, &author_user.id).await?;
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &author_user.id);
            reward.moderation_actions_performed += 1;
            reward.write(Path::new(&path)).await?;
        }

        {
            // Record the punishment
            let mut banned = UserData::modify_user_settings(&ctx.luro, &user_to_remove.id).await?;
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &user_to_remove.id);
            banned.moderation_actions.push(UserActions {
                action_type: vec![UserActionType::Ban],
                guild_id: Some(guild_id),
                reason,
                responsible_user: author_user.id
            });
            banned.write(Path::new(&path)).await?;
        }

        // If an alert channel is defined, send a message there
        ctx.luro
            .send_log_channel(&Some(guild_id), embed.clone(), crate::models::LuroLogChannel::Moderator)
            .await?;

        ctx.embed(embed.build())?.respond().await
    }
}