#[cfg(feature = "toml-driver")]
mod toml;
#[cfg(feature = "sqlx-driver")]
mod sqlx;

/// Luro's database, using the toml driver
#[cfg(feature = "toml-driver")]
#[derive(Clone, Debug)]
pub struct LuroDatabase {}

/// Luro's database, using the sqlx driver
#[cfg(feature = "sqlx-driver")]
#[derive(Clone, Debug)]
pub struct LuroDatabase(::sqlx::Pool<::sqlx::Postgres>);

#[derive(Clone)]
pub struct DatabaseGuild {
    pub guild_id: i64,
}