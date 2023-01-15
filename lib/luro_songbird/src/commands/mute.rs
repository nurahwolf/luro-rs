use luro_core::{Context, Error};
/// Mute the bot!
#[poise::command(slash_command, prefix_command, guild_only, ephemeral, category = "Music")]
pub async fn mute(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        let handler_lock = match ctx.data().songbird.get(guild_id) {
            Some(handler) => handler,
            None => {
                ctx.say("Not in a voice channel").await?;

                return Ok(());
            }
        };

        let mut handler = handler_lock.lock().await;

        if handler.is_mute() {
            ctx.say("Already muted").await?;
        } else {
            if let Err(e) = handler.mute(true).await {
                ctx.say(format!("Failed: {e:?}")).await?;
            }

            ctx.say("Now muted").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
