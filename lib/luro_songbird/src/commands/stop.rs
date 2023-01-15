use luro_core::{Context, Error};
/// Stop playing music!
#[poise::command(slash_command, prefix_command, guild_only, ephemeral, category = "Music")]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
            let handler = handler_lock.lock().await;
            let queue = handler.queue();
            queue.stop();

            ctx.say("Queue cleared.").await?;
        } else {
            ctx.say("Not in a voice channel to play in").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
