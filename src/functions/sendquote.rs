use poise::serenity_prelude::CreateEmbed;

use crate::{config::Quote, Context, Error};

use super::guild_accent_colour::guild_accent_colour;

pub async fn send_quote(ctx: Context<'_>, quote: &Quote, quote_id: Option<usize>) -> Result<(), Error> {
    let mut embed = CreateEmbed::default();
    embed.description(&quote.quote);
    embed.color(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, ctx.guild()));

    if let Ok(user) = ctx.serenity_context().http.get_user(quote.user_id).await {
        embed.author(|a| a.name(&user.name).icon_url(&user.face()));
    }

    if let Some(quote_id) = quote_id {
        embed.footer(|f| f.text(format!("Quote ID: {quote_id}")));
    }

    ctx.send(|b| {
        b.embed(|b| {
            *b = embed;
            b
        })
    })
    .await?;
    Ok(())
}