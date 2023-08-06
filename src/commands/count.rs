use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "count", desc = "Test to see if the framework is globally mutable")]
pub struct CountCommand {}

#[async_trait]
impl LuroCommand for CountCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let content;

        {
            let mut global_data = ctx.data_global.write();
            global_data.count += 1;
            content = format!("Here is your number: {}", global_data.count);
        }

        slash.content(content);
        ctx.respond(&mut slash).await
    }
}
