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
    pub owner_id: i64,
}

pub struct DatabaseInteraction {
    pub application_id: i64,
    pub interaction_id: i64,
    pub message_id: Option<i64>,
    pub data: Vec<u8>,
    pub kind: Vec<u8>,
    pub token: String,
}

pub struct DatabaseRole {
    pub role_id: i64
}

#[cfg(feature = "sqlx-driver")]
#[derive(Default, ::sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
}
pub struct DatabaseUser {
    pub name: String,
    pub user_id: i64,
    pub user_permissions: LuroUserPermissions,
}