use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::GuildChannel;

use luro_core::{Context, Error};

/// Create an Embed
#[poise::command(slash_command, prefix_command, category = "General")]
pub async fn embed(
    ctx: Context<'_>,
    #[description = "Title for the Embed"] title: String,
    #[description = "Content for the Embed"] content: String,
    #[description = "Channel to send the Embed"] channel: Option<GuildChannel>
) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    if let Some(channel) = channel {
        channel
            .send_message(ctx, |f| {
                f.embed(|e| {
                    e.title(title)
                        .description(content)
                        .color(guild_accent_colour(accent_colour, ctx.guild()))
                })
            })
            .await?;
    } else {
        ctx.send(|b| {
            b.embed(|b| {
                b.title(title)
                    .description(content)
                    .color(guild_accent_colour(accent_colour, ctx.guild()))
            })
        })
        .await?;
    }

    ctx.send(|builder| builder.content("Done!").ephemeral(true)).await?;

    Ok(())
}
