use anyhow::Error;

use twilight_gateway::stream::ShardRef;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    gateway::payload::outgoing::UpdateVoiceState,
    id::{marker::ChannelMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{functions::get_interaction_data, luro::Luro};

use super::create_response;

pub fn commands() -> Vec<Command> {
    vec![JoinCommand::create_command().into()]
}

#[derive(CommandModel, CreateCommand)]
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
    mut shard: ShardRef<'_>,
) -> Result<(), Error> {
    tracing::debug!(
        "join command in channel {} by {}",
        interaction.channel_id.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let data = JoinCommand::from_interaction(CommandInputData::from(
        *get_interaction_data(interaction).await?,
    ))?;

    let guild_id = interaction.guild_id.unwrap();

    let response =
        InteractionResponseDataBuilder::new().content(format!("Joined <#{}>!", data.channel));
    create_response(luro, interaction, response.build()).await?;

    shard
        .command(&UpdateVoiceState::new(
            guild_id,
            Some(data.channel),
            false,
            false,
        ))
        .await?;

    Ok(())
}
