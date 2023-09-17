#![feature(async_fn_in_trait)]
#![feature(let_chains)]

use anyhow::Context;
use dotenv::dotenv;
use events::event_handler;
use futures_util::StreamExt;
use luro_framework::Framework;
use luro_model::{configuration::Configuration, FILTER, INTENTS, LOG_PATH};
use std::env;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{
    fmt,
    prelude::__tracing_subscriber_SubscriberExt,
    reload::{self, Layer},
    util::SubscriberInitExt,
    Registry,
};
use twilight_gateway::{error::ReceiveMessageErrorType, stream::ShardEventStream};

mod commands;
mod events;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    // Database driver - Change this and the feature of `luro-database` to modify the driver!
    let database_driver = luro_database::toml::TomlDatabaseDriver::start().await?;
    let (filter, tracing_subscriber) = reload::Layer::new(FILTER);
    let config = Configuration::new(
        database_driver,
        INTENTS,
        env::var("LAVALINK_AUTHORISATION").context("Failed to get the variable LAVALINK_AUTHORISATION")?,
        env::var("LAVALINK_HOST").context("Failed to get the variable LAVALINK_HOST")?,
        env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?,
    )?
    .into();

    // Create the framework, Initialise tracing for logs based on bot name
    let (framework, mut shards) = Framework::new(config, tracing_subscriber).await?;
    init_tracing_subscriber(filter, &framework.database.current_user.read().unwrap().name);

    // Work on our events
    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
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

        tokio::spawn(event_handler(luro_framework::Context::new(framework.clone(), event, shard.latency().clone(), shard.sender())));
    }

    Ok(())
}

fn init_tracing_subscriber(filter: Layer<LevelFilter, Registry>, file_name: &String) {
    let file_appender = tracing_appender::rolling::hourly(LOG_PATH, format!("{file_name}.log"));
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let layer = fmt::layer().with_writer(non_blocking);
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .with(fmt::Layer::default())
        .init();
}
