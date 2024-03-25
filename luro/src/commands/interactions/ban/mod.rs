use luro_model::{
    database::Database,
    guild::Guild,
    response::{Punishment, PunishmentData},
    user::{MemberContext, User},
};
use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::UserMarker, Id},
};

use crate::models::interaction::{InteractionContext, InteractionError as Error, InteractionResult};

use super::{PunishmentPurgeAmount, PunishmentReason};

#[derive(CommandModel, CreateCommand)]
#[command(name = "ban", desc = "Ban a user", dm_permission = false)]
pub struct Ban {
    /// The user to ban
    pub user_id: Id<UserMarker>,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: PunishmentPurgeAmount,
    /// The reason they should be banned.
    pub reason: PunishmentReason,
    /// Some added description to why they should be banned
    pub details: Option<String>,
    /// Hide the banned message from chat, useful for discreet bans
    pub ephemeral: Option<bool>,
}

impl crate::models::CreateCommand for Ban {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        framework.ack_interaction(self.ephemeral.unwrap_or_default()).await?;

        let guild = framework.guild().await?;
        let twilight_client = &framework.gateway.twilight_client;

        let mut author = guild.member(framework.author_id()).await?;
        let mut bot = guild.member(framework.gateway.current_user.id).await?;
        let mut target = guild.user(self.user_id).await?;

        permission_check(&mut author, &mut bot, &framework.gateway.database, &guild, &mut target).await?;

        let reason = self.reason.fmt(self.details);
        let mut punishment = Punishment::Banned(
            PunishmentData {
                author: &author,
                target: &target,
                reason: &reason,
                guild: &guild,
                dm_successful: None,
            },
            self.purge.value(),
        );

        let target_dm = twilight_client.create_private_channel(target.user_id()).await;
        match target_dm {
            Ok(channel) => {
                let channel_id = channel.model().await?.id;
                let success = twilight_client
                    .create_message(channel_id)
                    .embeds(&[punishment.embed().into()])
                    .await;
                punishment.data().dm_successful = Some(success.is_ok())
            }
            Err(_) => punishment.data().dm_successful = Some(false),
        }

        framework.respond(|r| r.add_embed(punishment.embed())).await?;

        let ban = twilight_client.create_ban(guild.twilight_guild.id, target.user_id());
        let ban = ban.delete_message_seconds(self.purge.value() as u32);

        match reason.is_empty() {
            false => ban.await,
            true => ban.reason(&reason).await,
        }?;

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

        Ok(())
    }
}

async fn permission_check(
    author: &mut MemberContext,
    bot: &mut MemberContext,
    db: &Database,
    guild: &Guild<'_>,
    target: &mut User,
) -> InteractionResult<()> {
    // AUTHOR: Bypass checks if guild owner
    if author.user_id() != guild.twilight_guild.owner_id {
        tracing::debug!("Bypassing heirarchy checks as author is guild owner");
        return Ok(());
    }

    // Sync roles to ensure we have the most up-to-date data possible
    author.sync_roles(&db).await?;
    bot.sync_roles(&db).await?;
    if let User::Member(target) = target {
        target.sync_roles(&db).await?;
    }

    let (author_highest_role, author_permissions) = author.permission_matrix_highest_role(guild.twilight_guild.owner_id);
    let (bot_highest_role, bot_permissions) = bot.permission_matrix_highest_role(guild.twilight_guild.owner_id);

    // BOT: Missing permisisons
    if !bot_permissions.contains(Permissions::BAN_MEMBERS) {
        return Err(Error::BotMissingPermission(Permissions::BAN_MEMBERS));
    }

    // AUTHOR: Missing permisisons
    if !author_permissions.contains(Permissions::BAN_MEMBERS) {
        return Err(Error::MissingPermission(Permissions::BAN_MEMBERS));
    }

    // TARGET: Target is the guild owner
    if target.user_id() == guild.twilight_guild.owner_id {
        return Err(Error::ModifyServerOwner);
    }

    // Actual permission check
    if let User::Member(target) = target {
        if let Some(target_highest_role) = target.roles.first() {
            // Check bot is higher than the target
            if let Some(bot_highest_role) = bot_highest_role {
                if target_highest_role <= bot_highest_role {
                    return Err(Error::UserHeirarchy);
                }
            }

            // Check the author is higher than the victim
            if let Some(author_highest_role) = author_highest_role {
                if target_highest_role <= author_highest_role {
                    return Err(Error::BotHeirarchy);
                }
            }
        }
    }

    Ok(())
}
