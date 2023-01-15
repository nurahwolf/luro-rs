use luro_data::quotes::Quote;
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::CreateEmbed;

use luro_core::{Context, Error};


pub async fn send_quote(ctx: Context<'_>, quote: &Quote, quote_id: Option<usize>) -> Result<(), Error> {
    let mut embed = CreateEmbed::default();
    embed.description(&quote.quote);
    embed.color(guild_accent_colour(ctx.data().config.read().await.accent_colour, ctx.guild()));

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
