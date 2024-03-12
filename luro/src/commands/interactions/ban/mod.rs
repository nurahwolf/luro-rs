use crate::{
    embeds::{dm_sent, user_banned},
    models::{MemberContext, Role, User},
};
use twilight_http::request::AuditLogReason;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::UserMarker, Id},
};

use crate::models::interaction::{
    InteractionContext, InteractionError as Error, InteractionResult,
};

#[derive(
    twilight_interactions::command::CommandOption, twilight_interactions::command::CreateOption,
)]
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

#[derive(
    twilight_interactions::command::CommandOption,
    twilight_interactions::command::CreateOption,
    Debug,
    PartialEq,
)]
pub enum Reason {
    /// Someone who attempts to steal your money by offering fake commissions
    #[option(
        name = "Art Scam - Someone who attempts to steal your money by offering fake commissions",
        value = "art-scam"
    )]
    ArtScam,

    /// Compromised Account
    #[option(
        name = "Compromised Account - An account that has been token logged, or is spreading malware",
        value = "compromised"
    )]
    Compromised,

    /// Someone who is being a little bitch
    #[option(name = "Troll - Someone who is being a little bitch", value = "troll")]
    Troll,

    /// Someone who joined just to be a little bitch
    #[option(
        name = "Raider - Someone who joined just to be a little bitch",
        value = "raider"
    )]
    Raider,

    /// Racist, Sexist and other such things.
    #[option(
        name = "Vile - Racist, Sexist and other such plesent things.",
        value = ""
    )]
    Vile,

    /// A completely custom reason if the others do not fit
    #[option(
        name = "Custom Reason - A completely custom reason if the others do not fit",
        value = "custom"
    )]
    Custom,
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "ban", desc = "Ban a user", dm_permission = false)]
pub struct Ban {
    /// The user to ban
    pub user_id: Id<UserMarker>,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: TimeToBan,
    /// The reason they should be banned.
    pub reason: Reason,
    /// Some added description to why they should be banned
    pub details: Option<String>,
    /// Hide the banned message from chat, useful for discreet bans
    pub ephemeral: Option<bool>,
}

impl crate::models::CreateCommand for Ban {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        tracing::info!("acknowledge ineraction");
        framework
            .ack_interaction(self.ephemeral.unwrap_or_default())
            .await?;

        let twilight_client = &framework.gateway.twilight_client;
        let guild = framework.guild().await?;
        let author = framework.author_member(guild.id()).await?;
        let bot = framework.bot_member(guild.id()).await?;
        let target = framework.fetch_user(self.user_id).await?;
        tracing::info!("fetched data");

        let (author_highest_role, author_permissions) = author.permission_matrix();
        let (bot_highest_role, bot_permissions) = bot.permission_matrix();
        tracing::info!("fetched permissions");

        if !bot_permissions.contains(Permissions::BAN_MEMBERS) {
            return Err(Error::BotMissingPermission(Permissions::BAN_MEMBERS));
        }

        if !author_permissions.contains(Permissions::BAN_MEMBERS) {
            return Err(Error::MissingPermission(Permissions::BAN_MEMBERS));
        }

        if target.user_id() == author.guild_owner_id {
            return Err(Error::ModifyServerOwner);
        }

        permission_check(&author, &target, bot_highest_role, author_highest_role)?;
        tracing::info!("permissions checked");

        // Checks passed, now let's action the user
        let reason = reason(self.reason, self.details);
        tracing::info!("created reason");

        let mut embed = user_banned(
            &target,
            &author,
            reason.as_deref(),
            self.purge.value(),
            &guild.twilight_guild.name,
        )?;
        tracing::info!("created ban embed");

        let target_dm = twilight_client
            .create_private_channel(target.user_id())
            .await;
        tracing::info!("created dm");

        let dm_success = match target_dm {
            Ok(channel) => {
                let target_dm = channel.model().await?;
                twilight_client
                    .create_message(target_dm.id)
                    .embeds(&[embed.0.clone()])
                    .await
                    .is_ok()
            }
            Err(_) => false,
        };
        tracing::info!("dm attempt made");

        // Modify the original embed
        dm_sent(&mut embed, dm_success);
        tracing::info!("modified embed");

        framework.respond(|r| r.add_embed(embed)).await?;
        tracing::info!("responded to author");

        let ban = twilight_client.create_ban(guild.twilight_guild.id, target.user_id());
        let purge_seconds = self.purge.value() as u32;
        let ban = ban.delete_message_seconds(purge_seconds);
        tracing::debug!("Purging {purge_seconds:#?} seconds worth of messages!");

        match reason {
            None => ban.await,
            Some(ref reason) => ban.reason(reason).await,
        }?;
        tracing::info!("banned user");

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

fn reason(reason: Reason, details: Option<String>) -> Option<String> {
    let mut reason_string = match reason {
        Reason::ArtScam => "[Art Scam]".to_owned(),
        Reason::Compromised => "[Compromised Account]".to_owned(),
        Reason::Custom => String::new(),
        Reason::Raider => "[Raider]".to_owned(),
        Reason::Troll => "[Troll]".to_owned(),
        Reason::Vile => "[Vile]".to_owned(),
    };

    if let Some(details) = details {
        match reason == Reason::Custom {
            true => reason_string.push_str(&details.to_string()),
            false => reason_string.push_str(&format!(" - {details}")),
        }
    }

    match reason_string.is_empty() {
        true => None,
        false => Some(reason_string),
    }
}

fn permission_check(
    author: &MemberContext,
    target: &User,
    bot_highest_role: Option<Role>,
    author_highest_role: Option<Role>,
) -> InteractionResult<()> {
    // Check they are not overriding the role hierarchy, unless they are the server owner
    if author.guild_owner_id != author.user.user_id {
        tracing::info!("Bypassing heirarchy checks as user is guild owner");
        return Ok(());
    }

    if let Some(Some(target_highest_role)) = target.permission_matrix().map(|x| x.0) {
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

    Ok(())
}
