use luro_model::{
    database::Database,
    guild::Guild,
    response::{Punishment, PunishmentData},
    user::{MemberContext, User},
};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::UserMarker, Id},
};

use crate::models::interaction::{InteractionContext, InteractionError as Error, InteractionResult};

use super::PunishmentReason;

#[derive(CommandModel, CreateCommand)]
#[command(name = "warn", desc = "Warn a user", dm_permission = false)]
pub struct Command {
    /// The user to ban
    pub user_id: Id<UserMarker>,
    /// The reason they should be kicked.
    pub reason: PunishmentReason,
    /// Some added description to why they should be kicked
    pub details: Option<String>,
    /// Hide the banned message from chat, useful for discreet kicks
    pub ephemeral: Option<bool>,
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        framework.ack_interaction(self.ephemeral.unwrap_or_default()).await?;

        let guild = framework.guild().await?;
        let twilight_client = &framework.gateway.twilight_client;

        let mut author = guild.member(framework.author_id()).await?;
        let mut bot = guild.member(framework.gateway.current_user.id).await?;
        let mut target = guild.user(self.user_id).await?;

        permission_check(&mut author, &mut bot, &framework.gateway.database, &guild, &mut target).await?;

        let reason = self.reason.fmt(self.details);
        let mut punishment = Punishment::Warned(PunishmentData {
            author: &author,
            target: &target,
            reason: &reason,
            guild: &guild,
            dm_successful: None,
        });

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
    if !bot_permissions.contains(Permissions::KICK_MEMBERS) {
        return Err(Error::BotMissingPermission(Permissions::KICK_MEMBERS));
    }

    // AUTHOR: Missing permisisons
    if !author_permissions.contains(Permissions::KICK_MEMBERS) {
        return Err(Error::MissingPermission(Permissions::KICK_MEMBERS));
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
