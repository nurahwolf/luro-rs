use std::{sync::Arc, collections::HashMap};

use luro_database::LuroDatabase;
use twilight_gateway::{Event, Latency, MessageSender};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{Framework, Luro, LuroCommandType, LuroMutex};

/// Luro's primary context, which is instanced per event.
///
/// Contains [Framework] and houses data containing the [Event], [Latency] and [MessageSender].
#[derive(Clone)]
pub struct Context {
    /// The caching layer of the framework
    #[cfg(feature = "cache-memory")]
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    /// Luro's database, which accepts a driver that implements [LuroDatabaseDriver]
    pub database: Arc<LuroDatabase>,
    /// The raw event in which this [Context] was created from
    pub event: twilight_gateway::Event,
    /// A [Mutex] holding a bunch of [LuroCommandType] for global commands
    pub global_commands: LuroMutex<LuroCommandType>,
    /// A [Mutex] holding a bunch of [LuroCommandType] for guild commands, indexed by [GuildMarker]
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
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
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    /// Twilight client for interacting with the Discord API
    pub twilight_client: Arc<twilight_http::Client>,
}

impl Context {
    pub fn new(framework: Framework, event: Event, latency: Latency, shard: MessageSender) -> Self {
        Self {
            cache: framework.cache,
            database: framework.database,
            event,
            global_commands: framework.global_commands,
            guild_commands: framework.guild_commands,
            http_client: framework.http_client,
            latency,
            #[cfg(feature = "lavalink")]
            lavalink: framework.lavalink,
            shard,
            tracing_subscriber: framework.tracing_subscriber,
            twilight_client: framework.twilight_client,
        }
    }
}

impl Luro for Context {
    async fn interaction_client(&self) -> anyhow::Result<twilight_http::client::InteractionClient> {
        Ok(self.twilight_client.interaction(self.application().await?.id))
    }

    fn database(&self) -> std::sync::Arc<luro_database::LuroDatabase> {
        self.database.clone()
    }

    fn twilight_client(&self) -> std::sync::Arc<twilight_http::Client> {
        self.twilight_client.clone()
    }

    fn cache(&self) -> std::sync::Arc<twilight_cache_inmemory::InMemoryCache> {
        self.cache.clone()
    }
}
