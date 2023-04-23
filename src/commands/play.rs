use anyhow::Error;

use hyper::{Body, Request};
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};
use twilight_lavalink::{http::LoadedTracks, model::Play};
use twilight_model::application::{command::Command, interaction::Interaction};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{functions::get_interaction_data, luro::Luro};

use super::create_response;

pub fn commands() -> Vec<Command> {
    vec![PlayCommand::create_command().into()]
}

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "play",
    desc = "Give me a link or something and I'll try to play it",
    dm_permission = false
)]
pub struct PlayCommand {
    /// What you would like me to play
    song: String,
}

pub async fn play(
    luro: &Luro,
    interaction: &Interaction
) -> Result<(), Error> {
    tracing::debug!(
        "play command in channel {} by {}",
        interaction.channel_id.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let data = PlayCommand::from_interaction(CommandInputData::from(
        *get_interaction_data(interaction).await?,
    ))?;
    let guild_id = interaction.guild_id.unwrap();

    let player = luro.lavalink.player(guild_id).await.unwrap();
    let (parts, body) = twilight_lavalink::http::load_track(
        player.node().config().address,
        &data.song,
        &player.node().config().authorization,
    )?
    .into_parts();
    let req = Request::from_parts(parts, Body::from(body));
    let res = luro.hyper_client.request(req).await?;
    let response_bytes = hyper::body::to_bytes(res.into_body()).await?;

    let loaded = serde_json::from_slice::<LoadedTracks>(&response_bytes)?;
    let response;

    if let Some(track) = loaded.tracks.first() {
        player.send(Play::from((guild_id, &track.track)))?;

        let content = format!(
            "Playing **{:#?}** by **{:#?}**",
            track.info.title, track.info.author
        );

        response = InteractionResponseDataBuilder::new().content(content);
    } else {
        response = InteractionResponseDataBuilder::new().content("Didn't find any results");
    }

    create_response(luro, interaction, response.build()).await?;
    Ok(())
}
