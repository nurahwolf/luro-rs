use crate::{
    commands::quote::{quote_add::addquote, quote_get::get, quote_user::user},
    Command, Context, Error
};

mod quote_add;
mod quote_get;
mod quote_user;
mod story;

/// Get some information on things, like guilds and users.
#[poise::command(slash_command, category = "Guild", subcommands("addquote", "get", "user"))]
pub async fn quote(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn commands() -> [Command; 3] {
    [quote(), story::story(), quote_user::quote_user_context()]
}
