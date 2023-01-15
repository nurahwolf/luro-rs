use poise::serenity_prelude::User;
use rand::Rng;

use crate::{functions::sendquote::send_quote, Context, Error};

async fn get_user_quotes(ctx: Context<'_>, user: User) -> Result<(), Error> {
    let quotes = &ctx.data().quotes.read().await.quotes;

    let quotes_by_user = quotes
        .iter()
        .enumerate()
        .filter(|(_, quote)| quote.user_id == user.id.0)
        .collect::<Vec<_>>();
    if quotes_by_user.is_empty() {
        ctx.say("No quotes found for that user >:c\nTell them to shitpost more")
            .await?;
        return Ok(());
    }

    let random_number = rand::thread_rng().gen_range(0..quotes_by_user.len());

    // No quote specified, so lets get a random one instead!
    if let Some(quote) = quotes_by_user.get(random_number) {
        send_quote(ctx, quote.1, Some(random_number)).await?;
    } else {
        ctx.say("Failed to find a quote").await?;
    }

    Ok(())
}

/// Get random shit a user has said ;)
#[poise::command(slash_command, category = "Quotes")]
pub async fn user(ctx: Context<'_>, #[description = "User to get a random quote from"] user: User) -> Result<(), Error> {
    get_user_quotes(ctx, user).await?;
    Ok(())
}

/// Get random shit a user has said ;)
#[poise::command(context_menu_command = "Quotes by this user", category = "Quotes")]
pub async fn quote_user_context(ctx: Context<'_>, #[description = "User to get a random quote from"] user: User) -> Result<(), Error> {
    get_user_quotes(ctx, user).await?;
    Ok(())
}
