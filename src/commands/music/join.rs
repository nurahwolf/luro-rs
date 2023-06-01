use anyhow::Error;

use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    application::interaction::Interaction,
    gateway::payload::outgoing::UpdateVoiceState,
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::models::luro::Luro;

use super::create_response;

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
    luro: &Luro,
    interaction: &Interaction,
    shard: MessageSender,
    data: JoinCommand,
) -> Result<(), Error> {
    tracing::debug!(
        "join command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let response =
        InteractionResponseDataBuilder::new().content(format!("Joined <#{}>!", data.channel));
    create_response(luro, interaction, response.build()).await?;

    shard.command(&UpdateVoiceState::new(
        guild_id,
        Some(data.channel),
        false,
        false,
    ))?;

    Ok(())
}
