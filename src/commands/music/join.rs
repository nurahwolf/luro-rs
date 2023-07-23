use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    application::interaction::Interaction,
    gateway::payload::outgoing::UpdateVoiceState,
    id::{marker::ChannelMarker, Id}
};

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "join",
    desc = "Get me to join a voice channel to play some music!",
    dm_permission = false
)]
pub struct JoinCommand {
    /// The channel to join
    #[command(channel_types = "guild_voice guild_stage_voice")]
    channel: Id<ChannelMarker>
}

#[async_trait]
impl LuroCommand for JoinCommand {
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, shard: MessageSender) -> SlashResponse {
        let luro_response = ctx.defer_interaction(&interaction, false).await?;

        let guild_id = interaction.guild_id.unwrap();

        shard.command(&UpdateVoiceState::new(guild_id, Some(self.channel), false, false))?;

        Ok(InteractionResponse::Content {
            content: format!("Joined <#{}>!", self.channel),
            luro_response
        })
    }
}
