use async_trait::async_trait;
use hyper::{Body, Request};

use tracing::info;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::{
    http::{LoadType, LoadedTracks},
    model::Play
};

use crate::{models::LuroResponse, LuroContext};

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "play",
    desc = "Give me a link or something and I'll try to play it",
    dm_permission = false
)]
pub struct PlayCommand {
    /// What you would like me to play
    song: String
}

#[async_trait]
impl LuroCommand for PlayCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        let (parts, body) = twilight_lavalink::http::load_track(
            player.node().config().address,
            &self.song,
            &player.node().config().authorization
        )?
        .into_parts();
        let req = Request::from_parts(parts, Body::from(body));
        let res = ctx.hyper_client.request(req).await?;
        let response_bytes = hyper::body::to_bytes(res.into_body()).await?;
        let loaded = serde_json::from_slice::<LoadedTracks>(&response_bytes)?;

        let loadtype = match loaded.load_type {
            LoadType::LoadFailed => "LoadFailed",
            LoadType::NoMatches => "Failed",
            LoadType::SearchResult => "SearchResult",
            LoadType::TrackLoaded => "TrackLoaded",
            LoadType::PlaylistLoaded => "PlaylistLoaded",
            _ => "Unknown"
        };
        info!(loadtype);

        let content;
        if let Some(track) = loaded.tracks.first() {
            info!(track.track);

            player.send(Play::from((guild_id, &track.track)))?;

            content = if let (Some(title), Some(author)) = (&track.info.title, &track.info.author) {
                format!("Playing **{}** by **{}**", title, author)
            } else {
                format!("Playing **{:#?}** by **{:#?}**", track.info.title, track.info.author)
            };
        } else {
            content = "Didn't find any results".to_owned();
        }

        slash.content(content);
        ctx.respond(&mut slash).await
    }
}
