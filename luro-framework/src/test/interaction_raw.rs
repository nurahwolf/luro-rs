use std::{collections::HashMap, sync::Arc};

use luro_database::LuroDatabase;
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{LuroCommandType, LuroMutex};

/// A generic context spawned from an interaction
#[derive(Clone)]
pub struct RawInteraction {
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: Option<twilight_model::channel::Channel>,
    pub data: Option<twilight_model::application::interaction::InteractionData>,
    pub database: Arc<LuroDatabase<D>>,
    pub global_commands: LuroMutex<LuroCommandType>,
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    pub locale: Option<String>,
    pub member: Option<twilight_model::guild::PartialMember>,
    pub message: Option<twilight_model::channel::Message>,
    pub original: twilight_model::application::interaction::Interaction,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}
