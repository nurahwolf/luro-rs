use async_trait::async_trait;

use crate::{models::LuroResponse, LuroContext};
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand)]
#[command(name = "luro", desc = "Do things to me! Oh my...")]
pub enum LuroCommands {
    #[command(name = "nickname")]
    Nickname(LuroNicknameCommand)
}

#[async_trait]
impl LuroCommand for LuroCommands {
    async fn run_commands(self, ctx: &LuroContext, slash: LuroResponse) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Nickname(command) => command.run_command(ctx, slash).await
        }
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "nickname", desc = "Change my nickname! Or clear it.", dm_permission = false)]
pub struct LuroNicknameCommand {
    /// Set my nickname to this! Leave me blank to clear my nickname
    name: Option<String>
}

#[async_trait]
impl LuroCommand for LuroNicknameCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;

        ctx.twilight_client
            .update_current_member(guild_id)
            .nick(self.name.as_deref())
            .await?;
        slash.content("Done!").ephemeral();
        ctx.respond(&mut slash).await
    }
}
