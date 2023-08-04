use std::path::Path;

use crate::{
    models::{UserActionType, UserActions},
    traits::toml::LuroTOML,
    USERDATA_FILE_PATH
};
use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::guild::Permissions;
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    models::{GuildPermissions, LuroSlash, UserData},
    traits::luro_command::LuroCommand
};

use super::Reason;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "kick",
    desc = "Kick a user",
    dm_permission = false,
    default_permissions = "Self::default_permissions"
)]
pub struct KickCommand {
    /// The user to ban
    pub user: ResolvedUser,
    /// The reason they should be kicked.
    pub reason: Reason,
    /// Some added description to why they should be kicked
    pub details: Option<String>
}

#[async_trait]
impl LuroCommand for KickCommand {
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

        let user_to_remove = self.user.resolved;
        let member_to_remove = match self.user.member {
            Some(member) => member,
            None => return ctx.not_member_response(&user_to_remove.name).await
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
        let permissions = GuildPermissions::new(&ctx.luro.twilight_client, &guild_id).await?;
        let author_permissions = permissions.member(author.user.id, &author.roles).await?;
        let member_permissions = permissions.member(user_to_remove.id, &member_to_remove.roles).await?;
        let bot_permissions = permissions.current_member().await?;

        // Check if the author and the bot have required permissions.
        if member_permissions.is_owner() {
            return ctx.server_owner_response().await;
        }

        if !bot_permissions.guild().contains(Permissions::BAN_MEMBERS) {
            return ctx.bot_missing_permission_response(&"BAN_MEMBERS".to_owned()).await;
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

        // Checks passed, now let's action the user
        let user_to_ban_dm = match ctx.luro.twilight_client.create_private_channel(user_to_remove.id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => return ctx.kick_response(guild, author, user_to_remove, &reason, false).await
        };

        let mut embed = ctx
            .kick_embed(guild.clone(), author.clone(), user_to_remove.clone(), &reason)
            .await?;

        let victim_dm = ctx
            .luro
            .twilight_client
            .create_message(user_to_ban_dm.id)
            .embeds(&[embed.clone().build()])?
            .await;

        match victim_dm {
            Ok(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline()),
            Err(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        }

        ctx.luro
            .twilight_client
            .remove_guild_member(guild_id, user_to_remove.id)
            .await?;

        // If an alert channel is defined, send a message there
        ctx.luro
            .send_log_channel(&Some(guild_id), embed.clone(), crate::models::LuroLogChannel::Moderator)
            .await?;

        {
            let _ = UserData::modify_user_settings(&ctx.luro, &author.user.id).await?;
            let _ = UserData::modify_user_settings(&ctx.luro, &user_to_remove.id).await?;
            // Reward the person who actioned the ban
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &author.user.id);
            let data = &mut ctx
                .luro
                .user_data
                .get_mut(&author.user.id)
                .context("Expected to find user's data in the cache")?;
            data.moderation_actions_performed += 1;
            data.write(Path::new(&path)).await?;
            // Record the punishment
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &user_to_remove.id);
            let data = &mut ctx
                .luro
                .user_data
                .get_mut(&user_to_remove.id)
                .context("Expected to find user's data in the cache")?;
            data.moderation_actions.push(UserActions {
                action_type: vec![UserActionType::Kick],
                guild_id: Some(guild_id),
                reason,
                responsible_user: author.user.id
            });
            data.write(Path::new(&path)).await?;
        }

        ctx.embed(embed.build())?.respond().await
    }
}
