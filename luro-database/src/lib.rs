#[cfg(feature = "diesel-driver")]
mod diesel;
#[cfg(feature = "diesel-driver")]
mod schema;
#[cfg(feature = "sqlx-driver")]
mod sqlx;
#[cfg(feature = "toml-driver")]
mod toml;
mod types; // Added functionality around the types defined in this crate

#[cfg(feature = "diesel-driver")]
pub use crate::diesel::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};
#[cfg(feature = "sqlx-driver")]
pub use crate::sqlx::{
    application::{DbApplication, DbApplicationType},
    channel::{DbChannel, DbChannelType},
    guild::{DatabaseGuild, DatabaseGuildType},
    interaction::{DatabaseInteraction, DatabaseInteractionKind},
    luro_database::LuroDatabase,
    member::{DbMember, DbMemberType},
    message::{DatabaseMessage, DatabaseMessageSource, DatabaseMessageType},
    role::{DbRole, DbRoleType},
    user::{DatabaseUser, DatabaseUserType, LuroUserPermissions},
    user_marriage::DbUserMarriage,
    user_marriage_approvers::{DbUserMarriageApprovals, DbUserMarriageApprovalsCount}
};
#[cfg(feature = "toml-driver")]
pub use crate::toml::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};
