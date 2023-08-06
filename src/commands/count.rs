use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "count", desc = "Test to see if the framework is globally mutable")]
pub struct CountCommand {}

#[async_trait]
impl LuroCommand for CountCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let content;

        {
            let mut global_data = ctx.luro.global_data.write();
            global_data.count += 1;
            content = format!("Here is your number: {}", global_data.count);
        }

        ctx.content(content).respond().await
    }
}
