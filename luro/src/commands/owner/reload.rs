use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "reload",
    desc = "Reload data modified in local files. WARNING - This WILL overwrite data in memory!"
)]
pub struct ReloadCommand {}

#[async_trait]
impl LuroCommand for ReloadCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        ctx.content("Reloaded data from disk!".to_owned()).respond().await
    }
}
