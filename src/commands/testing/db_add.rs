use poise::serenity_prelude::Message;

use crate::{database::add_discord_message, Context, Error};

/// Add a message to the database
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn db_add(ctx: Context<'_>, #[description = "Message to add to DB"] msg: Message) -> Result<(), Error> {
    match add_discord_message(&ctx.data().database, msg.clone()) {
        Ok(_) => {
            ctx.say(format!("**Added message!**\nID: {}\nMessage:\n{}", &msg.id.0, &msg.content))
                .await?
        }
        Err(err) => ctx.say(format!("We had a fucky wucky!!{err}")).await?
    };

    Ok(())
}
