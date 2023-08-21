use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand)]
#[command(name = "luro", desc = "Do things to me! Oh my...")]
pub enum LuroCommands {
    #[command(name = "nickname")]
    Nickname(LuroNicknameCommand)
}

impl LuroCommand for LuroCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Nickname(command) => command.run_command(ctx).await
        }
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "nickname", desc = "Change my nickname! Or clear it.", dm_permission = false)]
pub struct LuroNicknameCommand {
    /// Set my nickname to this! Leave me blank to clear my nickname
    name: Option<String>
}

impl LuroCommand for LuroNicknameCommand {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await
        };

        ctx.framework
            .twilight_client
            .update_current_member(guild_id)
            .nick(self.name.as_deref())
            .await?;
        ctx.respond(|r| r.content("Done!!").ephemeral()).await
    }
}
