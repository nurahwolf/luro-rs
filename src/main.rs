use futures::StreamExt;
use luro::Luro;
use twilight_gateway::stream::ShardEventStream;

pub mod commands;
pub mod data;
pub mod event_handler;
pub mod functions;
pub mod luro;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise the tracing subscriber.
    tracing_subscriber::fmt::init();

    tracing::info!("Booting Luro!");
    let (luro, mut shards) = Luro::default().await?;

    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        match event {
            Ok(event) => {
                if let Err(why) = luro.clone().handle_event(event, shard).await {
                    tracing::warn!(?why, "error handling event");
                };
            }
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };
    }

    Ok(())
}
