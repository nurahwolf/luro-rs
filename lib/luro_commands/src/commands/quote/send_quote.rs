use luro_core::{Context, Error};
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::Message;

pub async fn send_quote(ctx: Context<'_>, message: Message, title: String, quote_id: Option<usize>) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let quote_id = match quote_id {
        Some(quote_id_specified) => quote_id_specified,
        None => ctx.data().quotes.read().await.quotes.len()
    };
    
    ctx.send(|b| {
        b.embed(|b| {
            b.author(|a| a.name(&message.author.name).icon_url(&message.author.face()))
                .title(title)
                .description(&message.content)
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .footer(|f| f.text(format!("Quote ID: {quote_id}")))
        })
    })
    .await?;

    Ok(())
}