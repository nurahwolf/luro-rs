mod fetch;

use std::sync::Arc;

use twilight_http::Client;

#[derive(Debug)]
pub struct Database {
    pub twilight_client: Arc<Client>,
}

impl Database {
    /// Create a new database instance
    pub fn new(twilight_client: Arc<Client>) -> Self {
        Self { twilight_client }
    }
}
