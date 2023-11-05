// Drivers
#[cfg(feature = "diesel-driver")]
mod diesel;
#[cfg(feature = "sqlx-driver")]
mod sqlx;
#[cfg(feature = "toml-driver")]
mod toml;

mod luro; // Functionality meant to be consumed by users of this crate. Names are prefixed with 'Luro'. // Standard Database functionality

pub mod sync; // Types used by the database drivers in order to sync data to the backend driver

pub use crate::luro::{
    luro_channel::LuroChannel, luro_character::LuroCharacter, luro_character::LuroCharacterFetish,
    luro_character_fetish::LuroCharacterFetishCategory, luro_character_image::LuroCharacterImage, luro_guild::LuroGuild,
    luro_guild_alert_channels::GuildAlertChannels, luro_guild_data::LuroGuildData, luro_image::LuroImage, luro_member::LuroMember,
    luro_role::LuroRole, luro_user::LuroUser, luro_user_data::LuroUserData, luro_user_type::LuroUserType, luro_word_count::WordCount,
    gender::Gender, sexuality::Sexuality, luro_member_data::LuroMemberData,
};

#[cfg(feature = "diesel-driver")]
pub use crate::diesel::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};
#[cfg(feature = "sqlx-driver")]
pub use crate::sqlx::{
    application::DbApplication,
    channel::DbChannel,
    interaction::{DatabaseInteraction, DatabaseInteractionKind},
    luro_database::LuroDatabase,
    member::DbMember,
    message::{DatabaseMessage, DatabaseMessageSource, DatabaseMessageType},
    role::{DbRole, DbRoleType},
    user::{DatabaseUser, LuroUserPermissions},
    user_marriage::DbUserMarriage,
    user_marriage_approvers::{DbUserMarriageApprovals, DbUserMarriageApprovalsCount},
};
#[cfg(feature = "toml-driver")]
pub use crate::toml::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};

/// Luro's Database. This struct takes driver modules to be able to generically store data on several types of backends.
/// Additionally, optional features for this crate can enable additional functionality, such as the twilight cache and twilight client.
/// Calls to new will always instance the database. Additional calls can be made to the building functions to setup for other features.
/// 
/// By default, this uses the Twilight client for updating the database with fresh data, and gracefully falling back to the API if the data does not exist.
/// If disabled, this will force the database to only query itself for data. Useful for if you can't reach the Discord API, however data will quickly grow stale. 
#[derive(Debug, Clone)]
pub struct LuroDatabaseV2<CACHE, DB> {
    /// The primary (caching) layer in which data is attempted to be fetched from first.
    /// Generally set to a cache in larger bots, or nothing in smaller bots.
    /// 
    /// Acceptable drivers include:
    /// NONE
    pub cache: CACHE,
    /// The secondary (database) layer in which data is attempted to be fetched from first.
    /// Generally set to the database driver in all instances.
    /// 
    /// Acceptable drivers include:
    /// SQLx (Postgress)
    pub secondary: DB,
    /// Twilight's client for interacting with the Discord API. Not available if the feature is disabled.
    pub twilight_client: std::sync::Arc<twilight_http::Client>,
    /// Twilight's in-memory cache

    pub pool: ::sqlx::Pool<::sqlx::Postgres>,
}