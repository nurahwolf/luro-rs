mod fetch;

use std::sync::Arc;

use twilight_http::Client;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Twilight failed to deserialize a response")]
    DeserializeBodyError(#[from] twilight_http::response::DeserializeBodyError),
    #[error("The API client had an error while communicating with the Discord API")]
    TwilightClient(#[from] twilight_http::Error),
}

#[derive(Debug)]
pub struct Database {
    pub twilight_client: std::sync::Arc<twilight_http::Client>,
}

impl Database {
    /// Create a new database instance
    pub fn new(twilight_client: Arc<Client>) -> Self {
        Self { twilight_client }
    }
}
