use luro_core::{Context, Error};
/// Undeafen the bot!
#[poise::command(slash_command, prefix_command, guild_only, ephemeral, category = "Music")]
pub async fn undeafen(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
            let mut handler = handler_lock.lock().await;
            if let Err(e) = handler.deafen(false).await {
                ctx.say(format!("Failed: {e:?}")).await?;
            }

            ctx.say("Undeafened").await?;
        } else {
            ctx.say("Not in a voice channel to undeafen in").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
