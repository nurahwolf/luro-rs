#![feature(let_chains)]

use futures_util::StreamExt;
use luro_framework::Framework;
use luro_model::types::Configuration;
use twilight_gateway::{error::ReceiveMessageErrorType, stream::ShardEventStream, Event};

// ===
// These variables are editable by the end user!
// ===
/// [tracing_subscriber] filter level
pub const FILTER: tracing_subscriber::filter::LevelFilter = tracing_subscriber::filter::LevelFilter::INFO;
// Luro's intents. Can be set to all, but rather spammy.
pub const INTENTS: twilight_gateway::Intents = twilight_gateway::Intents::all();

mod commands;
mod events;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // console_subscriber::init();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .thread_stack_size(4096 * 1024)
        .enable_all()
        .build()?;
    let config = Configuration::new(INTENTS - twilight_gateway::Intents::GUILD_PRESENCES).await?;
    tracing::info!("Hello World! I am alive!");

    // Create the framework, Initialise tracing for logs based on bot name
    let (framework, mut shards) = Framework::new(&config).await?;

    // Work on our events
    while let Some((shard, event)) = ShardEventStream::new(shards.iter_mut()).next().await {
        let event = match event {
            Err(error) => {
                if error.is_fatal() {
                    eprintln!("Gateway connection fatally closed, error: {error:?}");
                    break;
                }

                match error.kind() {
                    ReceiveMessageErrorType::Deserializing { event } => {
                        tracing::warn!("Failed to deserialise an object. Check DEBUG for the raw output.");
                        tracing::debug!(?event, "error while deserialising event");
                        continue;
                    }
                    _ => {
                        tracing::warn!("error while receiving event from discord. Check DEBUG for raw output.");
                        tracing::debug!(?error, "error while deserialising event");
                        continue;
                    }
                }
            }
            Ok(event) => event,
        };

        // Sync event to the database
        let (database, database_event) = (framework.database.clone(), event.clone());
        rt.spawn(async move { database.sync_gateway(database_event).await });

        // Pass the event to the appropiate handler
        let ctx: luro_framework::LuroContext =
            luro_framework::LuroContext::new(framework.clone(), event, shard.latency().clone(), shard.sender());
        rt.spawn(async move {
            let callback = match ctx.event.clone() {
                Event::InteractionCreate(event) => events::interaction_create::interaction_create_listener(ctx, event).await,
                Event::Ready(event) => events::ready::ready_listener(ctx, event).await,
                Event::MessageCreate(event) => events::message::create(ctx, event).await,
                Event::GuildCreate(event) => {
                    tracing::info!("guild_create - Joined guild {}", event.id);
                    Ok(())
                }
                Event::GuildDelete(event) => {
                    tracing::info!("guild_delete - Left guild {}", event.id);
                    Ok(())
                }
                _ => Ok(()),
            };

            if let Err(why) = callback {
                tracing::error!(why = ?why, "Unhandled error");
            }
        });

        // rt.spawn(events::event_handler(luro_framework::LuroContext::new(
        //     framework.clone(),
        //     event,
        //     shard.latency().clone(),
        //     shard.sender(),
        // )));
    }

    Ok(())
}
