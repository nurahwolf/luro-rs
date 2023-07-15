use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Volume;
use twilight_model::application::interaction::Interaction;

use crate::{framework::LuroFramework, interactions::InteractionResponse};

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
    ctx: &LuroFramework,
    interaction: &Interaction,
    data: VolumeCommand,
) -> anyhow::Result<InteractionResponse> {
    tracing::debug!(
        "volume command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let player = ctx.lavalink.player(guild_id).await.unwrap();
    player.send(Volume::from((guild_id, data.volume)))?;

    Ok(InteractionResponse::Text {
        content: format!("Set the volume to {}", data.volume),
        components: None,
        ephemeral: true,
    })
}
