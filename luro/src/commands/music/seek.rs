use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Seek;
use twilight_model::application::interaction::Interaction;

use crate::{framework::LuroFramework, interactions::InteractionResponse};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "seek", desc = "Seek through the track", dm_permission = false)]
pub struct SeekCommand {
    /// Where in the track do you want to seek to (in seconds)?
    position: i64,
}

pub async fn seek(
    ctx: &LuroFramework,
    interaction: &Interaction,
    data: SeekCommand,
) -> anyhow::Result<InteractionResponse> {
    tracing::debug!(
        "seek command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let player = ctx.lavalink.player(guild_id).await.unwrap();
    player.send(Seek::from((guild_id, data.position * 1000)))?;

    Ok(InteractionResponse::Text {
        content: format!("Seeked to {}s", data.position),
        components: None,
        ephemeral: true,
    })
}
