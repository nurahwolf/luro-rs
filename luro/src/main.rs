#![feature(let_chains)]

use std::env;

use anyhow::Context;
use futures_util::StreamExt;
use twilight_gateway::{stream::ShardEventStream, Intents};

use crate::{commands::Commands, framework::LuroFramework};

pub mod commands;
pub mod event_handler;
pub mod framework;
pub mod functions;
pub mod guild;
pub mod interactions;
pub mod macros;
pub mod permissions;
pub mod responses;

/// Luro's main accent colour
pub const ACCENT_COLOUR: u32 = 0xDABEEF;
/// Luro's DANGER colour
pub const COLOUR_DANGER: u32 = 0xD35F5F;
/// Transparent embed color (dark theme)
pub const COLOUR_TRANSPARENT: u32 = 0x2F3136;
/// Luro's SUCCESS colour
pub const COLOUR_SUCCESS: u32 = 0xA0D995;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise the tracing subscriber
    tracing_subscriber::fmt::init();
    tracing::info!("Booting Luro!");

    // Key things needed for the framework
    let (token, lavalink_host, lavalink_auth, intents) = (
        env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?,
        env::var("LAVALINK_HOST").context("Failed to get the variable LAVALINK_HOST")?,
        env::var("LAVALINK_AUTHORISATION")
            .context("Failed to get the variable LAVALINK_AUTHORISATION")?,
        Intents::GUILD_MESSAGES
            | Intents::GUILD_VOICE_STATES
            | Intents::MESSAGE_CONTENT
            | Intents::GUILD_INVITES,
    );

    let commands = Commands::default_commands();

    // Create the framework
    let (luro, mut shards) =
        LuroFramework::builder(commands, intents, lavalink_auth, lavalink_host, token).await?;
    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        match event {
            Err(error) => {
                if error.is_fatal() {
                    eprintln!("Gateway connection fatally closed, error: {error:?}");
                    break;
                }
            }
            Ok(event) => luro.handle_event(event, shard.sender()).await?,
        }
    }

    Ok(())
}
