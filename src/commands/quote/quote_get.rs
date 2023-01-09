use rand::Rng;

use crate::{functions::sendquote::send_quote, Context, Error};

/// Get random shit that someone has said, or get exactly what they said by quote ID
#[poise::command(slash_command, prefix_command, category = "Quotes")]
pub async fn get(ctx: Context<'_>, #[description = "Get a quote by ID"] quote: Option<usize>) -> Result<(), Error> {
    let quotes = &ctx.data().quotes.read().await.quotes;
    let random_number = rand::thread_rng().gen_range(0..quotes.len());

    // Try to get the specified quote
    if let Some(quote) = quote {
        if let Some(quote_resolved) = quotes.get(quote) {
            send_quote(ctx, quote_resolved, Some(quote)).await?;
            return Ok(());
        } else {
            ctx.say("Failed to get that quote! Sure you got the right ID?").await?;
            return Ok(());
        }
    }

    // No quote specified, so lets get a random one instead!
    if let Some(quote) = quotes.get(random_number) {
        send_quote(ctx, quote, Some(random_number)).await?;
    } else {
        ctx.say("Failed to find a quote").await?;
    }

    Ok(())
}
