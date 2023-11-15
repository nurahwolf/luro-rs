use hyper::{Body, Request};

use luro_framework::{responses::Response, CommandInteraction, ExecuteLuroCommand};
use tracing::info;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::{
    http::{LoadType, LoadedTracks},
    model::{Play, Volume},
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "play", desc = "Give me a link or something and I'll try to play it", dm_permission = false)]
pub struct PlayCommand {
    /// What you would like me to play
    song: String,
    /// Sets the volume between 0 and 1000! 10 is the default
    #[command(min_value = 0, max_value = 1_000)]
    volume: Option<i64>,
}

impl ExecuteLuroCommand for PlayCommand {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let volume = self.volume.unwrap_or(10);
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.response_simple(Response::NotGuild).await,
        };

        let player = ctx.lavalink.player(guild_id).await?;
        let (parts, body) =
            twilight_lavalink::http::load_track(player.node().config().address, &self.song, &player.node().config().authorization)?
                .into_parts();
        let req = Request::from_parts(parts, Body::from(body));
        let res = ctx.http_client.request(req).await?;
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
