#[cfg(feature = "database-sqlx")]
/// A module for fetching data using the SQLx driver.
pub mod sqlx;
/// A module for fetching data using the twilight HTTP client.
pub mod twilight;

#[cfg(feature = "database-sqlx")]
pub use sqlx::{Database, Error};

#[cfg(not(feature = "database-sqlx"))]
pub use twilight::{Database, Error};
