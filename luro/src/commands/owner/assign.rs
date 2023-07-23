use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    application::interaction::Interaction,
    id::{marker::RoleMarker, Id}
};

use crate::{interactions::InteractionResponse, responses::not_guild::not_guild_response, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "assign",
    desc = "Use the bot to assign a role to a user or self if not defined. Bypasses all other restrictions.",
    dm_permission = false
)]
pub struct AssignCommand {
    /// The role that should be assigned. It HAS to be below the bot for this to work.
    role: Id<RoleMarker>,
    /// Optionally the user to apply the role to. Applies to self if not defined.
    user: Option<ResolvedUser>
}

#[async_trait]
impl LuroCommand for AssignCommand {
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let (_, interaction_user, _) = self.interaction_context(&interaction, "owner assign")?;

        // User to action
        let user = if let Some(user) = self.user {
            user.resolved
        } else {
            interaction_user.clone()
        };

        // Guild to modify
        let guild_id = match interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return Ok(not_guild_response(Default::default()))
        };

        let response = ctx
            .twilight_client
            .add_guild_member_role(guild_id, user.id, self.role)
            .await?
            .status();

        Ok(InteractionResponse::Content {
            content: response.to_string(),
            luro_response: Default::default()
        })
    }
}
