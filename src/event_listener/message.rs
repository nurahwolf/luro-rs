use poise::{
    serenity_prelude::{Context, Message},
    FrameworkContext
};
use regex::Regex;

use crate::{
    constants::FURAFFINITY_REGEX, database::add_discord_message, functions::furaffinity::event_furaffinity, Data, Error
};

/// A Serenity listener for the [Message] type
pub async fn message(
    message: &Message,
    ctx: &Context,
    framework: &FrameworkContext<'_, Data, Error>,
    user_data: &Data
) -> Result<(), Error> {
    // Return if the sender was actually the bot
    if message.author.id == framework.bot_id {
        return Ok(());
    }

    // Add the message to the database
    match add_discord_message(&user_data.database, message.clone()) {
        Ok(_) => println!("Added message ID {} to database: {}", message.id.0, message.content),
        Err(err) => println!("Error while saving message to database: {err}")
    };

    // Run the furaffinity command if the message contains a link
    let regex = match Regex::new(FURAFFINITY_REGEX) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Message Listner: Failed to match the regex - {err}");
            return Ok(());
        }
    };
    if let Some(fa_match) = regex.find(&message.content) {
        match event_furaffinity(ctx, framework, message).await {
            Ok(_) => println!("Furaffinity: Regex matched - {}", fa_match.as_str()),
            Err(err) => println!("Furaffinity: Regex failed with the following message - {err}")
        }
    }
    Ok(())
}
