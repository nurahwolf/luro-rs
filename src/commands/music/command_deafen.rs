use crate::{Context, Error};

/// Get deafened nerd
#[poise::command(slash_command, prefix_command, guild_only, ephemeral, category = "Music")]
pub async fn deafen(ctx: Context<'_>) -> Result<(), Error> {
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

        if handler.is_deaf() {
            ctx.say("Already deafened").await?;
        } else {
            if let Err(e) = handler.deafen(true).await {
                ctx.say(format!("Failed: {e:?}")).await?;
            }

            ctx.say("Deafened").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
