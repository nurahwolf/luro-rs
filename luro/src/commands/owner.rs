use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{LuroContext, SlashResponse};

use super::LuroCommand;
use crate::responses::not_owner::not_owner_response;

use self::log::LogCommand;
use self::save::SaveCommand;

mod log;
mod save;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges uwu!")]
pub enum OwnerCommands {
    #[command(name = "save")]
    Save(SaveCommand),
    #[command(name = "log")]
    Log(LogCommand)
}

#[async_trait]
impl LuroCommand for OwnerCommands {
    async fn run_commands(self, interaction: Interaction, ctx: LuroContext, shard: MessageSender) -> SlashResponse {
        let (_, interaction_author, _) = self.interaction_context(&interaction, "owner command invoked")?;

        if !interaction_author.id.get() == 97003404601094144 {
            return Ok(not_owner_response());
        }

        // Call the appropriate subcommand.
        match self {
            Self::Save(command) => command.run_command(interaction, ctx, shard).await,
            Self::Log(command) => command.run_command(interaction, ctx, shard).await
        }
    }
}
