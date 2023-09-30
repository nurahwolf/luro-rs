use luro_model::configuration::Configuration;
use sqlx::{Error, postgres::PgPoolOptions};

use crate::LuroDatabase;

mod guilds;
mod interaction;
mod roles;
mod users;

impl LuroDatabase {
    pub async fn new(config: Configuration) -> Result<Self, Error> {
        Ok(Self(PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.connection_string)
                .await?))
    }
}