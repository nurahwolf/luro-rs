use std::path::Path;

use crate::{
    models::{LuroResponse, UserActionType, UserActions},
    traits::toml::LuroTOML,
    LuroContext, USERDATA_FILE_PATH
};

use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::guild::Permissions;
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    models::{GuildPermissions, UserData},
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
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;
        let (_, slash_author) = ctx.get_interaction_author(&slash)?;
        ctx.deferred(&mut slash).await?;

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
            slash.content("You need to specify a reason, dork!").ephemeral();
            return ctx.respond(&mut slash).await;
        }

        let user_to_remove = self.user.resolved;
        let member_to_remove = match self.user.member {
            Some(member) => member,
            None => return ctx.not_member_response(&user_to_remove.name, &mut slash).await
        };

        let guild = ctx.twilight_client.guild(guild_id).await?.model().await?;

        let author = ctx
            .twilight_client
            .guild_member(guild_id, slash_author.user_id)
            .await?
            .model()
            .await?;
        let permissions = GuildPermissions::new(&ctx.twilight_client, &guild_id).await?;
        let author_permissions = permissions.member(author.user.id, &author.roles).await?;
        let member_permissions = permissions.member(user_to_remove.id, &member_to_remove.roles).await?;
        let bot_permissions = permissions.current_member().await?;

        // Check if the author and the bot have required permissions.
        if member_permissions.is_owner() {
            return ctx.server_owner_response(&mut slash).await;
        }

        if !bot_permissions.guild().contains(Permissions::BAN_MEMBERS) {
            return ctx
                .bot_missing_permission_response(&"BAN_MEMBERS".to_owned(), &mut slash)
                .await;
        }

        // Check if the role hierarchy allow the author and the bot to perform
        // the ban.
        let member_highest_role = member_permissions.highest_role();

        if member_highest_role >= author_permissions.highest_role() {
            return ctx
                .user_hierarchy_response(&member_to_remove.nick.unwrap_or(user_to_remove.name.to_owned()), &mut slash)
                .await;
        }

        if member_highest_role >= bot_permissions.highest_role() {
            let global_data = ctx.data_global.read().clone();
            return ctx.bot_hierarchy_response(&global_data.current_user.name, &mut slash).await;
        }

        // Checks passed, now let's action the user
        let user_to_ban_dm = match ctx.twilight_client.create_private_channel(user_to_remove.id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => {
                return ctx
                    .kick_response(guild, author, user_to_remove, &reason, false, &mut slash)
                    .await
            }
        };

        let mut embed = ctx
            .kick_embed(guild.clone(), author.clone(), user_to_remove.clone(), &reason)
            .await?;

        let victim_dm = ctx
            .twilight_client
            .create_message(user_to_ban_dm.id)
            .embeds(&[embed.clone().build()])
            .await;

        match victim_dm {
            Ok(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline()),
            Err(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        }

        ctx.twilight_client.remove_guild_member(guild_id, user_to_remove.id).await?;

        // If an alert channel is defined, send a message there
        ctx.send_log_channel(&Some(guild_id), embed.clone(), crate::models::LuroLogChannel::Moderator)
            .await?;

        {
            let mut reward = UserData::modify_user_settings(ctx, &slash_author.user_id).await?;
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &slash_author.user_id);
            reward.moderation_actions_performed += 1;
            reward.write(Path::new(&path)).await?;
        }

        {
            // Record the punishment
            let mut warned = UserData::modify_user_settings(ctx, &user_to_remove.id).await?;
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &user_to_remove.id);
            warned.moderation_actions.push(UserActions {
                action_type: vec![UserActionType::Kick],
                guild_id: Some(guild_id),
                reason,
                responsible_user: author.user.id
            });
            warned.write(Path::new(&path)).await?;
        }

        slash.embed(embed.build())?;
        ctx.respond(&mut slash).await
    }
}
