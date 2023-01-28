use rand::Rng;

use luro_core::{Context, Error};

use crate::functions::sendquote::send_quote;

/// Get random shit that someone has said, or get exactly what they said by quote ID
#[poise::command(slash_command, prefix_command, category = "Quotes")]
pub async fn get(ctx: Context<'_>, #[description = "Get a quote by ID"] quote: Option<usize>) -> Result<(), Error> {
    let quotes = &ctx.data().quotes.read().await.quotes;
    let random_number = rand::thread_rng().gen_range(0..quotes.len());

    let returned_quote = match quote {
        Some(quote_defined) => quotes.get(quote_defined),
        None => quotes.get(random_number),
    };

    match returned_quote {
        Some(quote_resolved) => send_quote(ctx, quote_resolved, quote).await?,
        None => {
            ctx.say("Failed to get that quote! Sure you got the right ID?").await?;
            return Ok(());
        },
    };

    Ok(())
}
