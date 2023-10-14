mod data; // Added functionality around the types defined in this crate
#[cfg(feature = "diesel-driver")]
mod diesel;
#[cfg(feature = "diesel-driver")]
mod schema;
#[cfg(feature = "sqlx-driver")]
mod sqlx;
#[cfg(feature = "toml-driver")]
mod toml;

#[cfg(feature = "diesel-driver")]
pub use crate::diesel::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};
#[cfg(feature = "sqlx-driver")]
pub use crate::sqlx::{
    channel::{DbChannel, DbChannelType},
    guild::{DatabaseGuild, DatabaseGuildType},
    interaction::{DatabaseInteraction, DatabaseInteractionKind},
    luro_database::LuroDatabase,
    member::DbMember,
    message::{DatabaseMessage, DatabaseMessageSource, DatabaseMessageType},
    role::{DbRole, DbRoleType},
    user::{DatabaseUser, DatabaseUserType, LuroUserPermissions},
};
#[cfg(feature = "toml-driver")]
pub use crate::toml::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};
