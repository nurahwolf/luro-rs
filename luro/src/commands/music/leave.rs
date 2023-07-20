use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Destroy;
use twilight_model::{application::interaction::Interaction, gateway::payload::outgoing::UpdateVoiceState};

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "leave", desc = "Leave voice in a guild", dm_permission = false)]
pub struct LeaveCommand {}

impl LeaveCommand {
    pub async fn run(self, interaction: &Interaction, ctx: &LuroContext, shard: MessageSender) -> SlashResponse {
        let ephemeral = ctx.defer_interaction(interaction, true).await?;

        let guild_id = interaction.guild_id.unwrap();
        let player = ctx.lavalink.player(guild_id).await.unwrap();
        player.send(Destroy::from(guild_id))?;

        shard.command(&UpdateVoiceState::new(guild_id, None, false, false))?;
        Ok(InteractionResponse::Content {
            content: "Left the channel. Goodbye!".to_string(),
            ephemeral
        })
    }
}
