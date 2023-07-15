use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    application::interaction::Interaction,
    gateway::payload::outgoing::UpdateVoiceState,
    id::{marker::ChannelMarker, Id},
};

use crate::interactions::InteractionResponse;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "join",
    desc = "Get me to join a voice channel to play some music!",
    dm_permission = false
)]
pub struct JoinCommand {
    /// The channel to join
    #[command(channel_types = "guild_voice guild_stage_voice")]
    channel: Id<ChannelMarker>,
}

pub async fn join(
    interaction: &Interaction,
    shard: MessageSender,
    data: JoinCommand,
) -> anyhow::Result<InteractionResponse> {
    tracing::debug!(
        "join command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    shard.command(&UpdateVoiceState::new(
        guild_id,
        Some(data.channel),
        false,
        false,
    ))?;

    Ok(InteractionResponse::Text {
        content: format!("Joined <#{}>!", data.channel),
        components: None,
        ephemeral: true,
    })
}
