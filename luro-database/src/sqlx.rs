use luro_model::configuration::Configuration;
use sqlx::{postgres::PgPoolOptions, Error};

use crate::LuroDatabase;

mod channels;
mod guilds;
mod interactions;
mod messages;
mod roles;
mod users;

impl LuroDatabase {
    pub async fn new(config: &Configuration) -> Result<Self, Error> {
        Ok(Self(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.connection_string)
                .await?,
        ))
    }
}
