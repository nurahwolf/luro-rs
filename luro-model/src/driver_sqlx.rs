use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

use crate::configuration::Configuration;

pub mod guilds;
pub mod users;
pub mod roles;
pub mod interaction;

#[derive(Clone, Debug)]
pub struct PostgresDriver(Pool<Postgres>);

impl PostgresDriver {
    pub async fn new(config: Configuration) -> Result<Self, Error> {
        Ok(Self(PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.connection_string)
                .await?))
    }
}
