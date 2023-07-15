use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Stop;
use twilight_model::application::interaction::Interaction;

use crate::{framework::LuroFramework, interactions::InteractionResponse};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "stop",
    desc = "Stop the currently playing track",
    dm_permission = false
)]
pub struct StopCommand {}

pub async fn stop(
    ctx: &LuroFramework,
    interaction: &Interaction,
) -> anyhow::Result<InteractionResponse> {
    tracing::debug!(
        "stop command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let player = ctx.lavalink.player(guild_id).await.unwrap();
    player.send(Stop::from(guild_id))?;

    Ok(InteractionResponse::Text {
        content: "Stopped the track!".to_string(),
        components: None,
        ephemeral: true,
    })
}
