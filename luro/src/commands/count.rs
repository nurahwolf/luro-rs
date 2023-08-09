use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "count", desc = "Test to see if the framework is globally mutable")]
pub struct CountCommand {}

#[async_trait]
impl LuroCommand for CountCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let content;

        {
            match ctx.framework.database.count.write() {
                Ok(mut count) => {
                    *count += 1;
                    content = format!("Here is your number: {}", count);
                }
                Err(_) => content = "Count is poisoned :(".to_owned()
            };
        }

        ctx.content(content).respond().await
    }
}
