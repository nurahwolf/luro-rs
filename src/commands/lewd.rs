use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::builder::LuroResponseV2;
use crate::{LuroContext, SlashResponse};

use super::LuroCommand;

use self::muzzle::MuzzleCommand;

mod muzzle;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "lewd", desc = "Whoa! How very lewd of you! These are more... Adult orientated commands.")]
pub enum LewdCommands {
    #[command(name = "muzzle")]
    Muzzle(MuzzleCommand),
}

#[async_trait]
impl LuroCommand for LewdCommands {
    async fn run_commands(self, interaction: Interaction, ctx: LuroContext, shard: MessageSender) -> SlashResponse {
        let (interaction_channel, _, _) = self.interaction_context(&interaction, "owner command invoked")?;

        // TODO: Create a response type for this
        // TODO: Check for both 
        if let Some(nsfw) = interaction_channel.nsfw && !nsfw {
            return Ok(LuroResponseV2::new("lewd".to_owned(), &interaction).content("WHOA! You pervert!! This is a SFW CHANNEL!!".to_owned()).legacy_response(false))
        }

        if interaction_channel.nsfw.is_none() {
            return Ok(LuroResponseV2::new("lewd".to_owned(), &interaction).content("WHOA! You pervert!! This is a SFW CHANNEL!!".to_owned()).legacy_response(false))
        }

        // Call the appropriate subcommand.
        match self {
            Self::Muzzle(command) => command.run_command(interaction, ctx, shard).await
        }
    }
}
