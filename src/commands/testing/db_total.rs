use poise::serenity_prelude::User;

use crate::{database::total_messages_by_user, Context, Error, functions::guild_accent_colour::guild_accent_colour};

/// Total messages sent by a user
#[poise::command(prefix_command, slash_command, category = "Database")]
pub async fn db_total(ctx: Context<'_>, #[description = "User ID to get"] user: User) -> Result<(), Error> {
    let message_total = total_messages_by_user(&ctx.data().database, user.id.0);

    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .author(|author| author.name(&user.name).icon_url(&user.avatar_url().unwrap_or_default()))
                .color(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, ctx.guild()))
                .description(format!("**Total messages sent by user {}**\n{}", &user, message_total))
        })
    })
    .await?;

    Ok(())
}
