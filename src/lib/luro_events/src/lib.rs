use luro_core::{Error, FURAFFINITY_REGEX};
use luro_data::Data;
use luro_sled::add_discord_message;
use poise::{serenity_prelude::{Interaction, Message, Context, Ready, Activity, OnlineStatus}, FrameworkContext};
use regex::Regex;
use luro_furaffinity::poise_commands::event_furaffinity;

/// **Luro's event listener**
///
/// This function is called every time Discord pushes an event, which is then matched and reacted to accordingly.
pub async fn event_listener(
    ctx: &Context,
    event: &poise::Event<'_>,
    framework: poise::FrameworkContext<'_, Data, Error>,
    user_data: &Data
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => ready_listener(data_about_bot, ctx).await?,
        poise::Event::InteractionCreate { interaction } => interaction_create(interaction).await?,
        poise::Event::Message { new_message } => message(new_message, ctx, &framework, user_data).await?,
        poise::Event::PresenceUpdate { new_data: _ } => {} // Ignore this event
        poise::Event::TypingStart { event: _ } => {}       // Ignore this event
        poise::Event::GuildMemberUpdate {
            old_if_available: _,
            new: _
        } => {} // Ignore this event

        _ => {
            println!("Got an event in listener: {:?}", event.name());
        }
    }

    Ok(())
}

/// A Serenity listener for the [Ready] type
pub async fn ready_listener(ready: &Ready, ctx: &Context) -> Result<(), Error> {
    let http = &ctx.http;
    let api_version = ready.version;
    let bot_gateway = http.get_bot_gateway().await.unwrap();
    let t_sessions = bot_gateway.session_start_limit.total;
    let r_sessions = bot_gateway.session_start_limit.remaining;

    println!("Successfully logged into Discord as the following user:");
    println!("Bot username: {}", ready.user.tag());
    println!("Bot user ID: {}", ready.user.id);
    if let Ok(application_info) = http.get_current_application_info().await {
        println!("Bot owner: {}", application_info.owner.tag());
    }

    let guild_count = ready.guilds.len();

    println!("Connected to the Discord API (version {api_version}) with {r_sessions}/{t_sessions} sessions remaining.");
    println!("Connected to and serving a total of {guild_count} guild(s).");

    let presence_string = format!("on {guild_count} guilds | @luro help");
    ctx.set_presence(Some(Activity::playing(&presence_string)), OnlineStatus::Online)
        .await;
    Ok(())
}

/// A Serenity listener for the [poise::Event::InteractionCreate] type
pub async fn interaction_create(interaction: &Interaction) -> Result<(), Error> {
    match interaction.clone().application_command() {
        Some(interaction_command) => {
            println!("Event Listener: Data - {}", interaction_command.data.name)
        }
        None => println!("Event Listener: {}", interaction.id().0)
    };
    Ok(())
}

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
