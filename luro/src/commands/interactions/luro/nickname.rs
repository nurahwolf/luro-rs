use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(CommandModel, CreateCommand)]
#[command(name = "nickname", desc = "Change my nickname! Or clear it.", dm_permission = false)]
pub struct Command {
    /// Set my nickname to this! Leave me blank to clear my nickname
    name: Option<String>,
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return Err(crate::models::interaction::InteractionError::NotGuild),
        };

        ctx.gateway
            .twilight_client
            .update_current_member(guild_id)
            .nick(self.name.as_deref())
            .await?;
        ctx.respond(|r| r.content("Done!!").ephemeral()).await
    }
}
