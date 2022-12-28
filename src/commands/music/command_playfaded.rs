use std::time::Duration;

use crate::{
    commands::music::struct_music::{SongEndNotifier, SongFader},
    Context, Error
};
use songbird::{input::Restartable, Event, TrackEvent};
/// Play a song but fade it out every few seconds
#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn playfaded(ctx: Context<'_>, song: String) -> Result<(), Error> {
    if !song.starts_with("http") {
        ctx.say("Must provide a valid URL").await?;

        return Ok(());
    }

    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
            let mut handler = handler_lock.lock().await;

            let source = match Restartable::ytdl(song, true).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {why:?}");

                    ctx.say("Error sourcing ffmpeg").await?;

                    return Ok(());
                }
            };

            // This handler object will allow you to, as needed,
            // control the audio track via events and further commands.
            let song = handler.play_source(source.into());
            let send_http = ctx.serenity_context().http.clone();
            let chan_id = ctx.channel_id();

            // This shows how to periodically fire an event, in this case to
            // periodically make a track quieter until it can be no longer heard.
            let _ = song.add_event(Event::Periodic(Duration::from_secs(5), Some(Duration::from_secs(7))), SongFader { chan_id, http: send_http });

            // This shows how to fire an event once an audio track completes,
            // either due to hitting the end of the bytestream or stopped by user code.
            let send_http = ctx.serenity_context().http.clone();
            let _ = song.add_event(Event::Track(TrackEvent::End), SongEndNotifier { chan_id, http: send_http });

            ctx.say("Playing song").await?;
        } else {
            ctx.say("Not in a voice channel to play in").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
