use hyper::{Body, Request};

use tracing::info;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::{
    http::{LoadType, LoadedTracks},
    model::{Play, Volume},
};

use crate::interaction::LuroSlash;
use luro_model::database_driver::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "play",
    desc = "Give me a link or something and I'll try to play it",
    dm_permission = false
)]
pub struct PlayCommand {
    /// What you would like me to play
    song: String,
    /// Sets the volume between 0 and 1000! 10 is the default
    #[command(min_value = 0, max_value = 1000)]
    volume: Option<i64>,
}

impl LuroCommand for PlayCommand {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let volume = self.volume.unwrap_or(10);
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.framework.lavalink.player(guild_id).await.unwrap();
        let (parts, body) = twilight_lavalink::http::load_track(
            player.node().config().address,
            &self.song,
            &player.node().config().authorization,
        )?
        .into_parts();
        let req = Request::from_parts(parts, Body::from(body));
        let res = ctx.framework.hyper_client.request(req).await?;
        let response_bytes = hyper::body::to_bytes(res.into_body()).await?;
        let loaded = serde_json::from_slice::<LoadedTracks>(&response_bytes)?;

        let loadtype = match loaded.load_type {
            LoadType::LoadFailed => "LoadFailed",
            LoadType::NoMatches => "Failed",
            LoadType::SearchResult => "SearchResult",
            LoadType::TrackLoaded => "TrackLoaded",
            LoadType::PlaylistLoaded => "PlaylistLoaded",
            _ => "Unknown",
        };
        info!(loadtype);

        let content;
        if let Some(track) = loaded.tracks.first() {
            info!(track.track);

            player.send(Volume::from((guild_id, volume)))?;
            player.send(Play::from((guild_id, &track.track)))?;

            content = if let (Some(title), Some(author)) = (&track.info.title, &track.info.author) {
                format!("- Playing **{title}** by **{author}** - Volume {volume}")
            } else {
                format!(
                    "- Playing **{:#?}** by **{:#?}** - Volume {volume}",
                    track.info.title, track.info.author
                )
            };
        } else {
            content = "Didn't find any results".to_owned();
        }

        ctx.respond(|r| r.content(content)).await
    }
}
