use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "save", desc = "Flush data to disk")]
pub struct SaveCommand {}

#[async_trait]
impl LuroCommand for SaveCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        ctx.content("Flushed data to disk!".to_owned()).respond().await
    }
}
