use anyhow::Error;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Stop;
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::models::luro::Luro;

use super::create_response;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "stop",
    desc = "Stop the currently playing track",
    dm_permission = false
)]
pub struct StopCommand {}

pub async fn stop(luro: &Luro, interaction: &Interaction) -> Result<(), Error> {
    tracing::debug!(
        "stop command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let player = luro.lavalink.player(guild_id).await.unwrap();
    player.send(Stop::from(guild_id))?;

    let response = InteractionResponseDataBuilder::new().content("Stopped the track");

    create_response(luro, interaction, response.build()).await?;
    Ok(())
}
