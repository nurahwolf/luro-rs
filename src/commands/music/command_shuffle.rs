use crate::{Context, Error};
use rand::seq::SliceRandom;

/// Information on what is playing
#[poise::command(slash_command, prefix_command, guild_only, category = "Music")]
pub async fn shuffle(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
            let handler = handler_lock.lock().await;
            let handler_queue = handler.queue();
            handler_queue.modify_queue(|queue|
                queue.make_contiguous().shuffle(&mut rand::thread_rng())
            );
            ctx.say("Shuffled!").await?;
        } else {
            ctx.say("Not in a voice channel").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
