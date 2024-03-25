use std::sync::Arc;

use luro_model::{config::Config, database::Database};
use twilight_gateway::MessageSender;
use twilight_model::user::CurrentUser;

mod bot_name;
mod create_shards;
mod interaction_client;
mod register_commands;

#[derive(thiserror::Error, Debug)]
pub enum GatewayError {
    #[error("The database had an error.")]
    DatabaseError(#[from] luro_model::database::Error),
    #[error("The DISCORD_TOKEN environment variable was not present! You MUST pass this in order for me to start!")]
    NoToken,
    #[error("Twilight had an error while performing a HTTP request")]
    TwilightHTTP(#[from] twilight_http::Error),
    #[error("Twilight failed to convert a response into an item")]
    TwilightSerialization(#[from] twilight_http::response::DeserializeBodyError),
    #[error("The configuration had an error")]
    ConfigurationError(#[from] luro_model::config::Error),
    #[error("The embedded HTTP client had an error")]
    HTTPClientError(#[from] reqwest::Error),
    #[error("Failed to setup the correct number of shards")]
    ShardError(#[from] twilight_gateway::error::StartRecommendedError),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}

#[derive(Debug)]
pub struct Luro {
    pub application: twilight_model::oauth::Application,
    pub database: Database,
    pub shard: Option<MessageSender>,
    pub config: Config,
    pub twilight_client: Arc<twilight_http::Client>,
    pub current_user: Arc<CurrentUser>,
    pub http_client: reqwest::Client,
}
