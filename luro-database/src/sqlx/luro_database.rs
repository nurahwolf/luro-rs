#[derive(Clone, Debug)]
pub struct LuroDatabase(::sqlx::Pool<::sqlx::Postgres>);

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