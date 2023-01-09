use poise::serenity_prelude::Message;

use crate::{
    config::{Quote, Quotes},
    functions::guild_accent_colour::guild_accent_colour,
    Context, Error, QUOTES_FILE_PATH
};

/// Save the random shit a user has said;)
#[poise::command(context_menu_command = "Save this quote", category = "Quotes")]
pub async fn addquote(ctx: Context<'_>, #[description = "The quote which you wish to add to the database"] message: Message) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let quotes = &ctx.data().quotes.read().await.quotes;
    let mut new_quote = vec![Quote {
        user_id: *message.author.id.as_u64(),
        quote: String::from(&message.content)
    }];

    ctx.send(|b| {
        b.embed(|b| {
            b.author(|a| a.name(&message.author.name).icon_url(&message.author.face()))
                .title("Quote Added!")
                .description(&message.content)
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .footer(|f| f.text(format!("Quote ID: {}", quotes.len())))
        })
    })
    .await?;
    let quotes = &mut ctx.data().quotes.write().await;
    quotes.quotes.append(&mut new_quote);
    Quotes::write(quotes, QUOTES_FILE_PATH);

    Ok(())
}
