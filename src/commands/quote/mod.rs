use crate::{
    commands::quote::{command_quote_add::addquote, command_quote_get::get, command_quote_user::user},
    Command, Context, Error
};

mod command_quote_add;
mod command_quote_get;
mod command_quote_user;
mod command_story;
mod function_sendquote;

/// Get some information on things, like guilds and users.
#[poise::command(slash_command, category = "Guild", subcommands("addquote", "get", "user"))]
pub async fn quote(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn commands() -> [Command; 3] {
    [quote(), command_story::story(), command_quote_user::quote_user_context()]
}
