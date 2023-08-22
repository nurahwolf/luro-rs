use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_model::database::drivers::LuroDatabaseDriver;

use luro_model::{
    guild::log_channel::LuroLogChannel,
    user::{actions::UserActions, actions_type::UserActionType}
};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::guild::Permissions;

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
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let interaction = &ctx.interaction;
        let guild_id = interaction.guild_id.unwrap();
        let mut guild = ctx
            .framework
            .database
            .get_guild(&guild_id, &ctx.framework.twilight_client)
            .await?;
        let luro = ctx
            .framework
            .database
            .get_user(
                &ctx.framework.twilight_client.current_user().await?.model().await?.id,
                &ctx.framework.twilight_client
            )
            .await?;
        let mut moderator = ctx.get_interaction_author(interaction).await?;
        let mut punished_user = ctx
            .framework
            .database
            .get_user(&self.user.resolved.id, &ctx.framework.twilight_client)
            .await?;
        punished_user.update_user(&self.user.resolved);
        let mut response = ctx.acknowledge_interaction(false).await?;
        let moderator_permissions = guild.user_permission(&moderator)?;
        let moderator_highest_role = guild.user_highest_role(&moderator);
        let punished_user_highest_role = guild.user_highest_role(&punished_user);
        let luro_permissions = guild.user_permission(&luro)?;
        let luro_highest_role = guild.user_highest_role(&luro);
        let reason = reason(self.reason, self.details);

        if !luro_permissions.contains(Permissions::KICK_MEMBERS) {
            return ctx.bot_missing_permission_response(Permissions::KICK_MEMBERS).await;
        }

        if !moderator_permissions.contains(Permissions::KICK_MEMBERS) {
            return ctx.missing_permission_response(Permissions::KICK_MEMBERS).await;
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
        let mut embed: luro_builder::embed::EmbedBuilder =
            ctx.framework
                .kick_embed(&guild.name, &guild_id, &moderator, &punished_user, reason.as_deref());
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

        ctx.framework
            .twilight_client
            .remove_guild_member(guild_id, punished_user.id)
            .await?;

        moderator.moderation_actions_performed += 1;
        ctx.framework.database.save_user(&moderator.id, &moderator).await?;

        // Record the punishment
        punished_user.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Kick],
            guild_id: Some(guild_id),
            reason: reason.clone(),
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
