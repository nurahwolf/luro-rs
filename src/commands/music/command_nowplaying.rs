use crate::{Context, Error, utils::guild_accent_colour};
/// Information on what is playing
#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn nowplaying(ctx: Context<'_>, #[description = "Set to true so that only you see what is playing"] #[flag] ephemeral: bool) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
            let handler = handler_lock.lock().await;
            if let Some(track_handler) = handler.queue().current() {
                let metadata = track_handler.metadata();
                ctx.send(|builder|
                    builder.embed(|embed| {
                        embed.title("Now Playing");
                        embed.color(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, ctx.guild()));
                        if let Some(title) = &metadata.title {
                            embed.title(title);
                        }
                        if let Some(artist) = &metadata.artist {
                            embed.field("Arist", artist, false);
                        }
                        if let Some(date) = &metadata.date {
                            embed.field("Date", date, false);
                        }
                        if let Some(duration) = &metadata.duration {
                            embed.field("Duration", duration.as_secs(), false);
                        }
                        if let Some(source) = &metadata.source_url {
                            embed.field("Source", source, false);
                        }
                        if let Some(start_time) = &metadata.start_time {
                            embed.field("Start Time", start_time.as_secs(), false);
                        }
                        if let Some(track) = &metadata.track {
                            embed.field("Track", track, false);
                        }
                        if let Some(thumbnail) = &metadata.thumbnail {
                            embed.thumbnail(thumbnail);
                        }
                        embed
                    })
                    .ephemeral(ephemeral)
                    ).await?;
            } else {
                ctx.say("Failed to get track handler (Are you playing music?)").await?;
            }
        } else {
            ctx.say("Not in a voice channel").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
