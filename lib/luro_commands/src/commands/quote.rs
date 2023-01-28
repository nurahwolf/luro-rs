use luro_core::quotes::Quote;
use luro_core::{Context, Error};
use poise::serenity_prelude::Message;

use crate::commands::quote::get::get;
use crate::commands::quote::user::user;
use crate::commands::quote::send_quote::send_quote;
use crate::commands::quote::save_quote::save_quote;

mod get;
mod user;
mod save_quote;
mod send_quote;

/// Get some information on things, like guilds and users.
#[poise::command(
    context_menu_command = "Save this quote",
    slash_command,
    category = "Quotes",
    subcommands("get", "user")
)]
pub async fn quote(
    ctx: Context<'_>,
    #[description = "The quote which you wish to add to the database"] message: Message
) -> Result<(), Error> {
    let new_quote = vec![Quote {
        user_id: *message.author.id.as_u64(),
        quote: String::from(&message.content)
    }];

    send_quote(ctx, message, "Quote Added!".into(), None).await?;
    save_quote(ctx.data().quotes.clone(), new_quote).await;

    Ok(())
}