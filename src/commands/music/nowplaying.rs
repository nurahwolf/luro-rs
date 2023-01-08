use crate::{functions::nowplaying::now_playing, Context, Error};
/// Information on what is playing
#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn nowplaying(
    ctx: Context<'_>,
    #[description = "Set to true so that only you see what is playing"]
    #[flag]
    ephemeral: bool
) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
            let handler = handler_lock.lock().await;
            if let Some(track_handler) = handler.queue().current() {
                let metadata = track_handler.metadata();
                ctx.send(|builder| {
                    builder
                        .embed(|embed| {
                            *embed = now_playing(ctx.data().config.lock().unwrap().accent_colour, ctx.guild().unwrap(), None, metadata);
                            embed
                        })
                        .ephemeral(ephemeral)
                })
                .await?;
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