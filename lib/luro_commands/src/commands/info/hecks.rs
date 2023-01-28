use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::CreateEmbed;

use luro_core::{Context, Error};

/// Information about the heck commands!
#[poise::command(prefix_command, slash_command, category = "Guild")]
pub async fn hecks(ctx: Context<'_>) -> Result<(), Error> {
    let accent = ctx.data().config.read().await.accent_colour;
    let hecks = ctx.data().heck.read().await;
    let mut embed = CreateEmbed::default();
    embed.title("Heck Information");
    embed.colour(guild_accent_colour(accent, ctx.guild()));
    embed.field("SFW Hecks", format!("Heck Total: {}", hecks.sfw_hecks.len()), true);
    embed.field("NSFW Hecks", format!("Heck Total: {}", hecks.nsfw_hecks.len()), true);

    ctx.send(|builder| {
        builder.embed(|e| {
            *e = embed;
            e
        })
    })
    .await?;
    Ok(())
}
