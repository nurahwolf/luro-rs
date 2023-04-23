use anyhow::Error;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Stop;
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::luro::Luro;

use super::create_response;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "stop",
    desc = "Stop the currently playing track",
    dm_permission = false
)]
pub struct StopCommand {
    /// Sets the volume between 0 and 1000! 100 is the default
    #[command(min_value = 0, max_value = 1000)]
    volume: i64,
}

pub async fn stop(luro: &Luro, interaction: &Interaction) -> Result<(), Error> {
    tracing::debug!(
        "stop command in channel {} by {}",
        interaction.channel_id.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let player = luro.lavalink.player(guild_id).await.unwrap();
    player.send(Stop::from(guild_id))?;

    let response = InteractionResponseDataBuilder::new().content("Stopped the track");

    create_response(luro, interaction, response.build()).await?;
    Ok(())
}
