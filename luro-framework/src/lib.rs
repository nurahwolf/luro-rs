use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};

use luro_model::database::{drivers::LuroDatabaseDriver, LuroDatabase};
use twilight_model::{
    application::command::Command,
    id::{marker::GuildMarker, Id}
};

mod context;
mod framework;
mod interaction_context;
#[cfg(feature = "responses")]
pub mod responses;

/// The core framework. Should be available from ALL tasks and holds key data.
/// Context classes generally take a reference to this to perform their actions.
///
/// NOTE: This should be wrapped in an [Arc]!
pub struct Framework<D: LuroDatabaseDriver> {
    /// The caching layer of the framework
    #[cfg(feature = "cache-memory")]
    pub cache: twilight_cache_inmemory::InMemoryCache,
    /// Luro's database, which accepts a driver that implements [LuroDatabaseDriver]
    pub database: LuroDatabase<D>,
    /// HTTP client used for making outbound API requests
    #[cfg(feature = "http-client-hyper")]
    pub http_client: hyper::Client<hyper::client::HttpConnector>,
    /// Lavalink client, for playing music
    #[cfg(feature = "lavalink")]
    pub lavalink: twilight_lavalink::Lavalink,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Arc<twilight_http::Client>,
    /// The global tracing subscriber, for allowing manipulation within commands
    pub tracing_subscriber:
        tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    /// A mutable list of global commands, keyed by [String] (command name) and containing a [Command]
    pub global_commands: Mutex<HashMap<String, Command>>,
    /// A mutable list of guild commands, keyed by [GuildMarker] (command name) and containing a [Vec] of [Command]s
    pub guild_commands: Mutex<HashMap<Id<GuildMarker>, Vec<Command>>>
}

/// Luro's primary context, which is instanced per event.
/// More specific contexts are spawned where possible, such as [InteractionContext]
pub struct Context {
    /// [Latency] information about the connection to the gateway
    pub latency: twilight_gateway::Latency,
    /// A [MessageSender] used for communicating with the shard
    pub shard: twilight_gateway::MessageSender
}

/// A context sapwned only in which the event is an interaction
pub struct InteractionContext {
    /// The interaction this context was spawned from
    pub interaction: twilight_model::application::interaction::Interaction,
    /// [Latency] information about the connection to the gateway
    pub latency: twilight_gateway::Latency,
    /// A [MessageSender] used for communicating with the shard
    pub shard: twilight_gateway::MessageSender
}
