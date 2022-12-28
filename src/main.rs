#![feature(option_result_contains)]
#![feature(const_mut_refs)]
#![feature(let_chains)]
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::{Arc, Mutex}
};

use config::{Config, Heck, Quotes, Secrets, Stories};
use poise::serenity_prelude::{self as serenity, Activity, OnlineStatus};
use songbird::Songbird;

// Types
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
type Command = poise::Command<Data, Error>;

// Important Constants, may be manually updated!
// TODO: A way to do these progmatically
const DATA_PATH: &str = "data/"; // Consider setting this to XDG_DATA_HOME on a production system
const CONFIG_FILE_PATH: &str = "data/config.toml";
const HECK_FILE_PATH: &str = "data/heck.toml";
const QUOTES_FILE_PATH: &str = "data/quotes.toml";
const SECRETS_FILE_PATH: &str = "data/secrets.toml";
const STORIES_FILE_PATH: &str = "data/stories.toml";

// Structs
pub struct Data {
    config: config::Config,
    heck: config::Heck,
    quotes: config::Quotes,
    secrets: config::Secrets,
    stories: config::Stories,
    songbird: Arc<Songbird>,
    votes: Mutex<HashMap<String, u32>>
}

// Other modules
mod commands;
mod config;
mod utils;

// Finally at Luro!
// ===============

// We had a fucky wucky
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
            ctx.send(|message| message.ephemeral(true).content(format!("Error in command `{}`: {:?}", ctx.command().name, error)))
                .await
                .expect("Could not send error to channel!");
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}")
            }
        }
    }
}

async fn event_listener(_ctx: &serenity::Context, event: &poise::Event<'_>, _framework: poise::FrameworkContext<'_, Data, Error>, _user_data: &Data) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            let http = &_ctx.http;

            let api_version = data_about_bot.version;
            let bot_gateway = http.get_bot_gateway().await.unwrap();
            let t_sessions = bot_gateway.session_start_limit.total;
            let r_sessions = bot_gateway.session_start_limit.remaining;
            let bot_owner = http.get_current_application_info().await.unwrap().owner;

            println!("Successfully logged into Discord as the following user:");
            println!("Bot username: {}", data_about_bot.user.tag());
            println!("Bot user ID: {}", data_about_bot.user.id);
            println!("Bot owner: {}", bot_owner.tag());

            let guild_count = data_about_bot.guilds.len();

            println!("Connected to the Discord API (version {api_version}) with {r_sessions}/{t_sessions} sessions remaining.");
            println!("Connected to and serving a total of {guild_count} guild(s).");

            let presence_string = format!("on {guild_count} guilds | @luro help");
            _ctx.set_presence(Some(Activity::playing(&presence_string)), OnlineStatus::Online).await;
        }
        poise::Event::PresenceUpdate { new_data: _ } => {}

        _ => {
            println!("Got an event in listener: {:?}", event.name());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let songbird = songbird::Songbird::serenity();
    let data = Data {
        config: Config::get(CONFIG_FILE_PATH),
        heck: Heck::get(HECK_FILE_PATH),
        quotes: Quotes::get(QUOTES_FILE_PATH),
        secrets: Secrets::get(SECRETS_FILE_PATH),
        stories: Stories::get(STORIES_FILE_PATH),
        songbird: songbird.clone(),
        votes: Mutex::new(HashMap::new())
    };
    let token = match data.secrets.discord_token.clone() {
        Some(t) => t,
        None => std::env::var("DISCORD_TOKEN").expect("Congrats, you didn't set either DISCORD_TOKEN or include the token in the config. Terminating on your sheer stupidity.")
    };
    env::set_var("RUST_LOG", "info,poise_basic_queue=trace,poise=debug,serenity=debug");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            owners: HashSet::from([serenity::UserId(97003404601094144), serenity::UserId(203994450467225600), serenity::UserId(105716849336938496)]),
            on_error: |error| Box::pin(on_error(error)),
            event_handler: |ctx, event, framework, user_data| {
                Box::pin(async move {
                    event_listener(ctx, event, framework, user_data).await?;
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|_, _, _| Box::pin(async { Ok(data) }))
        .client_settings(move |f| f.voice_manager_arc(songbird))
        .token(token)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_MEMBERS | serenity::GatewayIntents::GUILD_PRESENCES
        );
    // .setup(move |_ctx, _ready, _framework| {
    //     Box::pin(async move {
    //         Ok(Data {
    //             config: read_config("config.toml"),
    //             songbird: songbird.clone()
    //         })
    //     })
    // });

    framework.run().await.unwrap();
}
