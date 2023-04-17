use luro_core::{Context, Error};
use poise::serenity_prelude::Mentionable;
use songbird::{Event, TrackEvent};

use crate::TrackStartNotifier;

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
    let accent_colour = ctx.data().config.read().await.accent_colour;
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;
        let voice_channel = guild.voice_states.get(&ctx.author().id);

        let handler_lock = match ctx.data().songbird.get(guild_id) {
            Some(handler) => handler,
            None => match voice_channel {
                Some(channel) => {
                    let (handle_lock, success) = ctx.data().songbird.join(guild_id, channel.channel_id.unwrap()).await;

                    if let Ok(_channel) = success {
                        ctx.say(&format!("Joined {}", channel.channel_id.unwrap().mention())).await?;
                        handle_lock
                    } else {
                        ctx.say("Error joining the channel").await?;
                        return Ok(());
                    }
                }
                None => {
                    ctx.say("Not in a voice channel").await?;
                    return Ok(());
                }
            }
        };

        let mut handler = handler_lock.lock().await;
        let send_http = ctx.serenity_context().http.clone();

        let source = if song.starts_with("http") {
            match songbird::input::ytdl(song).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {why:?}");
                    ctx.say(format!(
                        "Error matching YTDL input.\nReport the following error to Nurah:\n{why}"
                    ))
                    .await?;
                    return Ok(());
                }
            }
        } else {
            match songbird::input::ytdl_search(song).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {why:?}");
                    ctx.say(format!("Error searching YTDL.\nReport the following error to Nurah:\n{why}"))
                        .await?;
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
                accent_colour,
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

        ctx.say(format!("Added song to queue: position {}", handler.queue().len()))
            .await?;
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
