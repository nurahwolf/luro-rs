use anyhow::Error;

use twilight_gateway::stream::ShardRef;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Destroy;
use twilight_model::{
    application::interaction::Interaction, gateway::payload::outgoing::UpdateVoiceState,
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::luro::Luro;

use super::create_response;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "leave", desc = "Leave voice in a guild", dm_permission = false)]
pub struct LeaveCommand {}

pub async fn leave(
    luro: &Luro,
    interaction: &Interaction,
    mut shard: ShardRef<'_>,
) -> Result<(), Error> {
    tracing::debug!(
        "leave command in channel {} by {}",
        interaction.channel_id.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();
    let player = luro.lavalink.player(guild_id).await.unwrap();
    player.send(Destroy::from(guild_id))?;

    let response = InteractionResponseDataBuilder::new().content("Left the channel. Goodbye!");
    create_response(luro, interaction, response.build()).await?;

    shard
        .command(&UpdateVoiceState::new(guild_id, None, false, false))
        .await?;

    Ok(())
}
