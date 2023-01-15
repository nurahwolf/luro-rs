use luro_sled::total_messages_by_user;
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::User;

use luro_core::{Context, Error};

/// Total messages sent by a user
#[poise::command(prefix_command, slash_command, category = "Database")]
pub async fn db_total(ctx: Context<'_>, #[description = "User ID to get"] user: User) -> Result<(), Error> {
    let message_total = total_messages_by_user(&ctx.data().database, user.id.0);
    let accent_colour = ctx.data().config.read().await.accent_colour;

    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .author(|author| author.name(&user.name).icon_url(&user.avatar_url().unwrap_or_default()))
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .description(format!("**Total messages sent by user {}**\n{}", &user, message_total))
        })
    })
    .await?;

    Ok(())
}
