use async_trait::async_trait;
use luro_framework::{command::ExecuteLuroCommand, CommandInteraction};
use twilight_interactions::command::{CommandModel, CreateCommand};

use self::add::Add;

mod add;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "fetish", desc = "Add or remove some fetishes to your profile")]
pub enum Fetish {
    #[command(name = "add")]
    Add(Add),
}
#[async_trait]
impl ExecuteLuroCommand for Fetish {
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        match self {
            Self::Add(command) => command.interaction_command(ctx).await,
        }
    }
}
