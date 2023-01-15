use luro_core::{Context, Error};
/// Set the volume of the bot - 100 is 100%
#[poise::command(slash_command, prefix_command, guild_only, ephemeral, category = "Music")]
pub async fn volume(ctx: Context<'_>, mut volume: f32) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;
        volume /= 100.0;

        if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
            let handler = handler_lock.lock().await;
            if let Err(e) = handler.queue().current().unwrap().set_volume(volume) {
                ctx.say(format!("Failed: {e:?}")).await?;
            }

            ctx.say(format!("Set the volume to: {volume:?}")).await?;
        } else {
            ctx.say("Not in a voice channel to set the current volume").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
