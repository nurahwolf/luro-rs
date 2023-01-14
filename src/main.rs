#![feature(option_result_contains)]
use std::{
    collections::HashSet,
    env,
    sync::{atomic::AtomicUsize, Arc}
};

use config::{Config, Heck, Quotes, Secrets, Stories};
use constants::{BOT_TOKEN, CONFIG_FILE_PATH, DATABASE_FILE_PATH, FURAFFINITY_REGEX, HECK_FILE_PATH, QUOTES_FILE_PATH, SECRETS_FILE_PATH, STORIES_FILE_PATH};
use poise::{
    serenity_prelude::{GatewayIntents, UserId},
    FrameworkOptions
};
use sled::Db;
use songbird::Songbird;
use tokio::sync::RwLock;
use tracing_subscriber::FmtSubscriber;

// Types
/// Luro's error type
type Error = Box<dyn std::error::Error + Send + Sync>;
/// Luro's context, which allows the user to grab the serenity context + data struct
type Context<'a> = poise::Context<'a, Data, Error>;
/// A wrapped around the Poise command context, for ease of use.
type Command = poise::Command<Data, Error>;

// Modules
mod commands;
mod config;
mod constants; // **NOTE:** This file is intended to be USER EDITABLE! Please refer to it to modify key ways Luro operates!
mod database;
mod event_listener;
mod functions;
mod structs;

// Structs

/// **Luro's Data**
///
/// A struct holding all user data that makes Luro tick.
///
/// This structure intends to use best practices <https://github.com/serenity-rs/serenity/blob/current/examples/e12_global_data/src/main.rs>, for example `Arc<RwLock<HashMap<String, u64>>>` for read/write, and `Arc<AtomicUsize>` for read only data.
pub struct Data {
    /// Configuration that is got from the "config.toml" file. This is intended to be user modifiable and easy, by non-technically inclined users.
    /// NOTE: There is "constants.rs" where a bunch of other 'config' like variables live, however these are intended for ADVANCED USERS, hence they live here.
    config: Arc<RwLock<Config>>,
    /// Luro's Database, which is currently a sled.rs instance.
    database: Arc<Db>,
    /// Heck: A bunch of silly messages to throw at a user. This refers to the "heck.toml" file on disk.
    heck: Arc<RwLock<Heck>>,
    /// Quotes: A bunch of silly messages that people have said. This refers to the "quotes.toml" file on disk.
    quotes: Arc<RwLock<Quotes>>,
    /// Application secrets got from the "secrets.toml" file on disk.
    secrets: Arc<Secrets>,
    /// Stories: A bunch of 'stories', which are more shitposty in nature. This refers to the "stories.toml" file on disk.
    stories: Arc<RwLock<Stories>>,
    /// A Songbird instance for voice fun.
    songbird: Arc<Songbird>,
    /// The total commands that have been ran in this instance. NOTE: This is RESET when the bot restarts! It only lives in memory.
    command_total: Arc<RwLock<AtomicUsize>>
}

// We are finally at Luro!
// ===============

/// **Luro's error handler**
///
/// This function is called every time we have an error. There are many types of errors, so we only handle the ones we are particularly interested in. The rest get forwarded to the default error handler.
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
            ctx.send(|message| message.ephemeral(true).content(format!("Error in command `{}`: {:?}", ctx.command().name, error)))
                .await
                .expect("Could not send error to channel!");
        }
        // We are not interested in this particular error, so handle it by the built-in function.
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}")
            }
        }
    }
}

/// **Luro's entry function**
///
/// This is a thread wrapped in `tokio::main` to async main, and from here sets up the rest of Luro.
#[tokio::main]
async fn main() {
    // Luro's initialised songbird context
    let songbird = songbird::Songbird::serenity();
    // Luro's initialised data context
    let data = Data {
        config: RwLock::new(Config::get(CONFIG_FILE_PATH)).into(),
        database: sled::open(DATABASE_FILE_PATH).expect("Could not open / create database").into(),
        heck: RwLock::new(Heck::get(HECK_FILE_PATH)).into(),
        quotes: RwLock::new(Quotes::get(QUOTES_FILE_PATH)).into(),
        secrets: Secrets::get(SECRETS_FILE_PATH).into(),
        stories: RwLock::new(Stories::get(STORIES_FILE_PATH)).into(),
        songbird: songbird.clone(),
        command_total: RwLock::new(AtomicUsize::new(0)).into() // NOTE: Resets to zero on bot restart, by design
    };

    // Attempt to get a token from `secrets.toml`. If it does not exist, try to get it from the environment variable defined by [BOT_TOKEN].
    // If that ALSO does not exist, insult the user for being incompetent.
    let token = match data.secrets.discord_token.clone() {
        Some(t) => t,
        None => match std::env::var(BOT_TOKEN) {
            Ok(environment_token) => environment_token,
            Err(err) => panic!("Congrats, you didn't set either {BOT_TOKEN} or include the token in the config file. Terminating on your sheer stupidity.\n{err}")
        }
    };

    // Extra logging, honestly no clue what it does lol
    env::set_var("RUST_LOG", "info,poise_basic_queue=trace,poise=debug,serenity=debug");
    let subscriber = FmtSubscriber::builder().with_target(false).finish();
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => println!("Loaded tracing subscriber"),
        Err(_) => panic!("Failed to load tracing subscriber!")
    };

    // Framework Options
    let mut framework_options = FrameworkOptions {
        commands: commands::commands(),
        on_error: |error| Box::pin(on_error(error)),
        event_handler: |ctx, event, framework, user_data| {
            Box::pin(async move {
                event_listener::event_listener(ctx, event, framework, user_data).await?;
                Ok(())
            })
        },
        pre_command: |ctx| {
            Box::pin(async move {
                *ctx.data().command_total.write().await.get_mut() += 1;
            })
        },
        ..Default::default()
    };

    // If owners are configured, override the framework options with said owner
    if let Some(owners) = &data.secrets.owners {
        let owners_map: HashSet<UserId> = owners.iter().map(|user_id| UserId(*user_id)).collect();
        framework_options.owners = owners_map;
    };

    // Actually start the framework!
    let framework = poise::Framework::builder()
        .options(framework_options)
        .setup(|_, _, _| Box::pin(async { Ok(data) }))
        .client_settings(move |f| f.voice_manager_arc(songbird))
        .token(token)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_PRESENCES);

    framework.run().await.unwrap();
}
