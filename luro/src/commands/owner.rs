use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{LuroContext, SlashResponse, BOT_OWNER};

use super::LuroCommand;
use crate::responses::not_owner::not_owner_response;

use self::assign::AssignCommand;
use self::log::LogCommand;
use self::modify_role::ModifyRoleCommand;
use self::save::SaveCommand;

mod assign;
mod log;
mod modify_role;
mod save;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges uwu!")]
pub enum OwnerCommands {
    #[command(name = "save")]
    Save(SaveCommand),
    #[command(name = "log")]
    Log(LogCommand),
    #[command(name = "assign")]
    Assign(Box<AssignCommand>),
    #[command(name = "modify_role")]
    Modify(ModifyRoleCommand)
}

#[async_trait]
impl LuroCommand for OwnerCommands {
    async fn run_commands(self, interaction: Interaction, ctx: LuroContext, shard: MessageSender) -> SlashResponse {
        let (_, interaction_author, _) = self.interaction_context(&interaction, "owner command invoked")?;

        if !interaction_author.id.get() == BOT_OWNER {
            return Ok(not_owner_response(Default::default()));
        }

        // Call the appropriate subcommand.
        match self {
            Self::Save(command) => command.run_command(interaction, ctx, shard).await,
            Self::Log(command) => command.run_command(interaction, ctx, shard).await,
            Self::Assign(command) => command.run_command(interaction, ctx, shard).await,
            Self::Modify(command) => command.run_command(interaction, ctx, shard).await
        }
    }
}
