use hyper::{Body, Request};

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::{http::LoadedTracks, model::Play};
use twilight_model::application::interaction::Interaction;

use crate::{interactions::InteractionResponse, LuroContext};

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

impl PlayCommand {
    pub async fn run(self, interaction: &Interaction, ctx: &LuroContext) -> anyhow::Result<InteractionResponse> {
        let ephemeral = ctx.defer_interaction(interaction, true).await?;

        let guild_id = interaction.guild_id.unwrap();

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

        let content;
        if let Some(track) = loaded.tracks.first() {
            player.send(Play::from((guild_id, &track.track)))?;

            content = if let (Some(title), Some(author)) = (&track.info.title, &track.info.author) {
                format!("Playing **{}** by **{}**", title, author)
            } else {
                format!("Playing **{:#?}** by **{:#?}**", track.info.title, track.info.author)
            };
        } else {
            content = "Didn't find any results".to_owned();
        }

        Ok(InteractionResponse::Content {
            content,
            ephemeral,
            deferred: true
        })
    }
}
