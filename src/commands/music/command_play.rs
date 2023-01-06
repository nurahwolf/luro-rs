use songbird::{Event, TrackEvent};

use crate::{commands::music::struct_music::TrackStartNotifier, Context, Error};
/// Queue up a song to play!
#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    song: String,
    volume: Option<f32>,
    #[description = "Loop this song until it is skipped"]
    #[flag]
    play_looped: bool
) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
            let mut handler = handler_lock.lock().await;
            let send_http = ctx.serenity_context().http.clone();
            let config = ctx.data().config.lock().unwrap().clone();

            let source = if song.starts_with("http") {
                match songbird::input::ytdl(song).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Err starting source: {why:?}");
                        ctx.say("Error sourcing ffmpeg").await?;
                        return Ok(());
                    }
                }
            } else {
                match songbird::input::ytdl_search(song).await {
                    Ok(source) => source,
                    Err(why) => {
                        println!("Err starting source: {why:?}");
                        ctx.say("Error sourcing ffmpeg").await?;
                        return Ok(());
                    }
                }
            };

            let track_handler = handler.enqueue_source(source);
            if play_looped {
                track_handler.enable_loop()?;
            }
            track_handler.add_event(
                Event::Track(TrackEvent::Play),
                TrackStartNotifier {
                    chan_id: ctx.channel_id(),
                    http: send_http,
                    config,
                    guild: ctx.guild().unwrap(),
                    user: ctx.author().clone()
                }
            )?;

            match volume {
                Some(mut vol) => {
                    vol /= 100.0;
                    track_handler.set_volume(vol)?;
                }
                None => {
                    track_handler.set_volume(0.2)?;
                }
            }

            ctx.say(format!("Added song to queue: position {}", handler.queue().len())).await?;
        } else {
            ctx.say("Not in a voice channel to play in").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
