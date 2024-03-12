#![feature(string_remove_matches)]
#![feature(let_chains)]

pub mod builders;
pub mod commands;
pub mod database;
pub mod embeds;
mod error_handler;
mod gateway;
#[cfg(any(
    feature = "logs-stdout",
    feature = "logs-file",
    feature = "logs-tokio-console"
))]
mod logging;
pub mod models;
pub mod responses;

/// The intents from the gateway we want to listen / respond to. Consider removing presence updates as it can be rather spammy.
const INTENTS: twilight_gateway::Intents = twilight_gateway::Intents::all();
/// Luro's main accent colour
pub const ACCENT_COLOUR: u32 = 0xDABEEF;
/// Luro's DANGER colour
pub const COLOUR_DANGER: u32 = 0xD35F5F;
/// Transparent embed color (dark theme)
pub const COLOUR_TRANSPARENT: u32 = 0x2F3136;
/// Luro's SUCCESS colour
pub const COLOUR_SUCCESS: u32 = 0xA0D995;
/// The primary owner user ID. Used for some defaults, as well as to say who owns the bot. This MUST  be set, even if a group of people own Luro, as its used as a fallback for when data is not tied to a specific user. For example, see [Story].
pub const PRIMARY_BOT_OWNER: twilight_model::id::Id<twilight_model::id::marker::UserMarker> =
    twilight_model::id::Id::new(373524896187416576);
/// Luro's primary owner(s)
pub const BOT_OWNERS: [twilight_model::id::Id<twilight_model::id::marker::UserMarker>; 4] = [
    twilight_model::id::Id::new(1138489661187182692), // Zeron
    // twilight_model::id::Id::new(1146227925960638474), // Ferrona
    twilight_model::id::Id::new(138791390279630849), // Tzat
    twilight_model::id::Id::new(261308783546859520), // Aurora
    twilight_model::id::Id::new(373524896187416576), // Nurah
];

/// When true, the bot will shutdown.
pub static SHUTDOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    #[cfg(any(
        feature = "logs-stdout",
        feature = "logs-file",
        feature = "logs-tokio-console"
    ))]
    let _guards = logging::init_logging(); // Start the logging service first, for understandable reasons
    let (gateway, shards) =
        gateway::Gateway::create_shards(INTENTS - twilight_gateway::Intents::GUILD_PRESENCES)
            .await?;
    let mut senders = Vec::with_capacity(shards.len()); // A collection of senders, used to communicate with the shards
    let mut tasks = Vec::with_capacity(shards.len()); // A collection of tasks, which is used to gracefully close the bot

    // For each shard, spawn a thread to handle it
    for shard in shards {
        senders.push(shard.sender());
        tasks.push(tokio::spawn(gateway::shard_runner(gateway.clone(), shard)));
    }

    // Wait till we receive the shutdown signal, then start the shutdown process
    tokio::signal::ctrl_c().await?;
    SHUTDOWN.store(true, std::sync::atomic::Ordering::Relaxed);
    for sender in senders {
        if let Err(why) = sender.close(twilight_gateway::CloseFrame::NORMAL) {
            tracing::error!(?why, "Failed to tell shard to shutdown!")
        }
    }

    // Now that the shards have been told to shut down, send that request
    for join_handler in tasks {
        _ = join_handler.await;
    }

    // All should be good now, exit Luro!
    Ok(())
}
