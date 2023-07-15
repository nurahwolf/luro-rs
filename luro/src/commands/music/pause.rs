use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Pause;
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{framework::LuroFramework, interactions::InteractionResponse};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "pause",
    desc = "Pause / Unpause the current playing music",
    dm_permission = false
)]
pub struct PauseCommand {}

pub async fn pause(
    ctx: &LuroFramework,
    interaction: &Interaction,
) -> anyhow::Result<InteractionResponse> {
    tracing::debug!(
        "pause command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();

    let player = ctx.lavalink.player(guild_id).await.unwrap();
    let paused = player.paused();
    player.send(Pause::from((guild_id, !paused)))?;

    let action = if paused { "Unpaused " } else { "Paused" };
    let _response = InteractionResponseDataBuilder::new().content(format!("{action} the track"));

    Ok(InteractionResponse::Text {
        content: format!("{action} the track"),
        components: None,
        ephemeral: false,
    })
}
