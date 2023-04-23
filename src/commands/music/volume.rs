use anyhow::Error;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Volume;
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::luro::Luro;

use super::create_response;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "volume",
    desc = "Set the volume of the player!",
    dm_permission = false
)]
pub struct VolumeCommand {
    /// Sets the volume between 0 and 1000! 100 is the default
    #[command(min_value = 0, max_value = 1000)]
    volume: i64,
}

pub async fn volume(
    luro: &Luro,
    interaction: &Interaction,
    data: VolumeCommand,
) -> Result<(), Error> {
    tracing::debug!(
        "volume command in channel {} by {}",
        interaction.channel_id.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let player = luro.lavalink.player(guild_id).await.unwrap();
    player.send(Volume::from((guild_id, data.volume)))?;

    let response =
        InteractionResponseDataBuilder::new().content(format!("Set the volume to {}", data.volume));

    create_response(luro, interaction, response.build()).await?;
    Ok(())
}
