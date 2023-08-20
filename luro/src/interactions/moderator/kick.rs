use luro_model::{
    guild_permissions::GuildPermissions, luro_log_channel::LuroLogChannel, user_actions::UserActions,
    user_actions_type::UserActionType
};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{guild::Permissions, id::Id};

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
    pub details: Option<String>
}

impl LuroCommand for Kick {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
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

        // Permission checks
        if reason.is_empty() {
            response.content("You need to specify a reason, dork!").ephemeral();
            return ctx.send_respond(response).await;
        }

        if !bot_permissions.guild().contains(Permissions::KICK_MEMBERS) {
            return ctx.bot_missing_permission_response(&"KICK_MEMBERS".to_owned()).await;
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
        let mut embed = ctx.kick_embed(&guild, &punished_user, &reason).await?;
        let punished_user_dm = match ctx.framework.twilight_client.create_private_channel(punished_user_id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => return ctx.kick_response(&guild, &punished_user, &reason, false).await
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

        ctx.framework
            .twilight_client
            .remove_guild_member(guild_id, punished_user_id)
            .await?;

        author_user.moderation_actions_performed += 1;
        ctx.framework.database.modify_user(&author_user_id, &author_user).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Kick],
            guild_id: Some(guild_id),
            reason,
            responsible_user: author_user_id
        });
        ctx.framework.database.modify_user(&punished_user_id, &punished_user).await?;

        // If an alert channel is defined, send a message there
        ctx.framework
            .send_log_channel(&Some(guild_id), embed.into(), LuroLogChannel::Moderator)
            .await?;

        Ok(())
    }
}
