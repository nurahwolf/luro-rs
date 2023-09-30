use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "count", desc = "Test to see if the framework is globally mutable")]
pub struct CountCommand {}

impl LuroCommand for CountCommand {
    async fn run_command(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let content;

        {
            match ctx.framework.database.count.write() {
                Ok(mut count) => {
                    *count += 1;
                    content = format!("Here is your number: {}", count);
                }
                Err(_) => content = "Count is poisoned :(".to_owned(),
            };
        }

        ctx.respond(|r| r.content(content)).await
    }
}
