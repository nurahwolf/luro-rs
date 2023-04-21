use hyper::{Body, Request};

use twilight_lavalink::{http::LoadedTracks, model::Play};
use twilight_model::{channel::Message, gateway::payload::incoming::MessageCreate};

use crate::State;

pub async fn play(msg: Box<MessageCreate>, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "play command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .twilight_client
        .create_message(msg.channel_id)
        .content("What's the URL of the audio to play?")?
        .await?;

    let author_id = msg.author.id;
    let msg = state
        .twilight_standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let guild_id = msg.guild_id.unwrap();

    let player = state.lavalink.player(guild_id).await.unwrap();
    let (parts, body) = twilight_lavalink::http::load_track(
        player.node().config().address,
        &msg.content,
        &player.node().config().authorization,
    )?
    .into_parts();
    let req = Request::from_parts(parts, Body::from(body));
    let res = state.hyper_client.request(req).await?;
    let response_bytes = hyper::body::to_bytes(res.into_body()).await?;

    let loaded = serde_json::from_slice::<LoadedTracks>(&response_bytes)?;

    if let Some(track) = loaded.tracks.first() {
        player.send(Play::from((guild_id, &track.track)))?;

        let content = format!(
            "Playing **{:?}** by **{:?}**",
            track.info.title, track.info.author
        );
        state
            .twilight_client
            .create_message(msg.channel_id)
            .content(&content)?
            .await?;
    } else {
        state
            .twilight_client
            .create_message(msg.channel_id)
            .content("Didn't find any results")?
            .await?;
    }

    Ok(())
}
