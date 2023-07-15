use anyhow::Error;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    framework::LuroFramework, interactions::InteractionResponse, responses::text::say::say,
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "count",
    desc = "Test to see if the framework is globally mutable"
)]
pub struct CountCommand {}

impl CountCommand {
    pub async fn run(self, ctx: &LuroFramework) -> Result<InteractionResponse, Error> {
        let message;

        {
            let mut global_data = ctx.global_data.write();
            global_data.count += 1;
            message = format!("Here is your number: {}", global_data.count);
        }

        Ok(say(message, None, false))
    }
}
