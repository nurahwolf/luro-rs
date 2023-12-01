use std::sync::Arc;

use luro_database::Database;
use twilight_gateway::{Event, Latency, MessageSender};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{Framework, Luro};

/// Luro's primary context, which is instanced per event.
///
/// Contains [Framework] and houses data containing the [Event], [Latency] and [MessageSender].
#[derive(Debug, Clone)]
pub struct LuroContext {
    /// Luro's database, which accepts a driver that implements [LuroDatabaseDriver]
    pub database: Arc<Database>,
    /// The raw event in which this [Context] was created from
    pub event: twilight_gateway::Event,
    /// A HTTP client used for making web requests. Uses Hyper.
    #[cfg(feature = "http-client-hyper")]
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    /// Latency information from the shard that this event was spawned from
    pub latency: twilight_gateway::Latency,
    /// A lavalink instance for playing music
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    /// A [MessageSender] for interacting with the shard
    pub shard: twilight_gateway::MessageSender,
    /// Tracing subscriber information
    pub logging: Arc<luro_logging::Logging>,
    /// Twilight client for interacting with the Discord API
    pub twilight_client: Arc<twilight_http::Client>,
}

impl LuroContext {
    pub fn new(framework: Framework, event: Event, latency: Latency, shard: MessageSender) -> Self {
        Self {
            database: framework.database,
            event,
            http_client: framework.http_client,
            latency,
            #[cfg(feature = "lavalink")]
            lavalink: framework.lavalink,
            shard,
            logging: framework.logging,
            twilight_client: framework.twilight_client,
        }
    }
}

impl Luro for LuroContext {
    async fn interaction_client(&self) -> anyhow::Result<twilight_http::client::InteractionClient> {
        Ok(self.twilight_client.interaction(self.application().await?.id))
    }

    fn database(&self) -> std::sync::Arc<luro_database::Database> {
        self.database.clone()
    }

    fn twilight_client(&self) -> std::sync::Arc<twilight_http::Client> {
        self.twilight_client.clone()
    }

    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        None
    }
}
