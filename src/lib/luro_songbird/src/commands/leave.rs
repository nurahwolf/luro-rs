use luro_core::{Context, Error};
/// Leave the voice channel!
#[poise::command(slash_command, prefix_command, guild_only, ephemeral, category = "Music")]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(guild) = ctx.guild() {
        let guild_id = guild.id;

        let has_handler = ctx.data().songbird.get(guild_id).is_some();

        if has_handler {
            if let Err(e) = ctx.data().songbird.remove(guild_id).await {
                ctx.say(format!("Failed: {e:?}")).await?;
            }

            ctx.say("Left voice channel").await?;
        } else {
            ctx.say("Not in a voice channel").await?;
        }
    } else {
        ctx.say("You need to be in a guild for me to play music!").await?;
    }

    Ok(())
}
