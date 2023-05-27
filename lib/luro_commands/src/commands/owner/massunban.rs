use luro_core::{Context, Error};
use poise::serenity_prelude::{Member, Mentionable};

/// Unban EVERYONE in a guild
#[poise::command(slash_command, prefix_command, guild_only, category = "Owner")]
pub async fn massunban(ctx: Context<'_>) -> Result<(), Error> {
    let guild = match ctx.guild() {
        Some(ok) => ok,
        None => {
            ctx.say("I'm not in a guild you dork").await?;
            return Ok(());
        }
    };

    let bans = match guild.bans(ctx).await {
        Ok(ok) => ok,
        Err(why) => {
            ctx.say(format!("Failed to get a list of bans: {why}")).await?;
            return Ok(());
        }
    };

    let total_bans = bans.len();

    let handle = ctx.say("Processing...").await.unwrap();

    for ban in bans {
        guild.unban(ctx, ban.user.id).await;
    }

    handle.edit(ctx, |f| f.content(format!("Yeeted {total_bans} bans!"))).await;

    Ok(())
}
