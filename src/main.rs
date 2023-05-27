use luro_commands::commands;
use luro_core::initialise_data;
use std::{collections::HashSet, env};

use luro_core::BOT_TOKEN;
use luro_events::{event_listener, on_error::on_error};
use poise::{
    serenity_prelude::{GatewayIntents, UserId},
    FrameworkOptions
};

pub const LURO_GIT: &str = env!("CARGO_MANIFEST_DIR");

/// **Luro's entry function**
///
/// This is a thread wrapped in `tokio::main` to async main, and from here sets up the rest of Luro.
#[tokio::main]
async fn main() {
    // Luro's initialised data context
    let data = initialise_data().await;
    let songbird = data.songbird.clone();

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
    env::set_var("RUST_LOG", "debug,poise_basic_queue=trace,poise=debug,serenity=debug");
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => println!("Loaded tracing subscriber"),
        Err(_) => panic!("Failed to load tracing subscriber!")
    };

    // Framework Options
    let mut framework_options = FrameworkOptions {
        commands: commands(),
        on_error: |error| Box::pin(on_error(error)),
        event_handler: |ctx, event, framework, user_data| {
            Box::pin(async move {
                event_listener(ctx, event, framework, user_data).await?;
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

    // Return the framework!
    match poise::Framework::builder()
        .options(framework_options)
        .setup(|_, _, _| Box::pin(async { Ok(data) }))
        .client_settings(move |f| f.voice_manager_arc(songbird))
        .token(token)
        .intents(
            GatewayIntents::non_privileged()
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_PRESENCES
        )
        .run_autosharded()
        .await
    {
        Ok(_) => println!("Luro has started!"),
        Err(err) => panic!("Luro just crashed: {err}")
    };
}
