#![feature(async_fn_in_trait)]
#![feature(let_chains)]

use futures_util::StreamExt;
use luro_framework::Framework;
use luro_model::configuration::Configuration;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, reload::Layer, Registry};
use twilight_gateway::{error::ReceiveMessageErrorType, stream::ShardEventStream};

// ===
// These variables are editable by the end user!
// ===
/// [tracing_subscriber] filter level
pub const FILTER: tracing_subscriber::filter::LevelFilter = tracing_subscriber::filter::LevelFilter::INFO;
// Luro's intents. Can be set to all, but rather spammy.
pub const INTENTS: twilight_gateway::Intents = twilight_gateway::Intents::all();
/// The log path. By default this is a sub directory of DATA_PATH
pub const LOG_PATH: &str = "data/log/";

mod commands;
mod events;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .thread_stack_size(4096 * 1024)
        .enable_all()
        .build()?;
    let config = Configuration::new(INTENTS, FILTER)?;

    // Create the framework, Initialise tracing for logs based on bot name
    let (framework, mut shards) = Framework::new(&config).await?;
    init_tracing_subscriber(config.filter, &framework.twilight_client.current_user().await?.model().await?.name);

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
                        tracing::warn!("Failed to deserialise an object. Check DEBUG for the raw output");
                        tracing::debug!(?event, "error while deserialising event");
                        continue;
                    }
                    _ => {
                        tracing::warn!(?error, "error while receiving event");
                        continue;
                    }
                }
            }
            Ok(event) => event,
        };

        rt.spawn(events::event_handler(luro_framework::LuroContext::new(
            framework.clone(),
            event,
            shard.latency().clone(),
            shard.sender(),
        )));

        // tokio::spawn(events::event_handler(luro_framework::Context::new(
        //     framework.clone(),
        //     event,
        //     shard.latency().clone(),
        //     shard.sender(),
        // )));
    }

    Ok(())
}

fn init_tracing_subscriber(filter: Layer<tracing_subscriber::filter::LevelFilter, Registry>, file_name: &String) {
    let file_appender = tracing_appender::rolling::hourly(LOG_PATH, format!("{file_name}.log"));
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let layer = fmt::layer().with_writer(non_blocking);
    tracing_subscriber::util::SubscriberInitExt::init(
        tracing_subscriber::registry().with(filter).with(layer).with(fmt::Layer::default()),
    );
}
