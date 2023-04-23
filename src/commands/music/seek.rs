use anyhow::Error;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Seek;
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::luro::Luro;

use super::create_response;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "seek", desc = "Seek through the track", dm_permission = false)]
pub struct SeekCommand {
    /// Where in the track do you want to seek to (in seconds)?
    position: i64,
}

pub async fn seek(luro: &Luro, interaction: &Interaction, data: SeekCommand) -> Result<(), Error> {
    tracing::debug!(
        "seek command in channel {} by {}",
        interaction.channel_id.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let player = luro.lavalink.player(guild_id).await.unwrap();
    player.send(Seek::from((guild_id, data.position * 1000)))?;

    let response =
        InteractionResponseDataBuilder::new().content(format!("Seeked to {}s", data.position));

    create_response(luro, interaction, response.build()).await?;
    Ok(())
}
