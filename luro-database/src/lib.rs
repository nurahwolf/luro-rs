// Drivers
#[cfg(feature = "diesel-driver")]
mod diesel;
#[cfg(feature = "sqlx-driver")]
mod sqlx;
#[cfg(feature = "toml-driver")]
mod toml;

mod database;
mod luro; // Functionality meant to be consumed by users of this crate. Names are prefixed with 'Luro'. // Standard Database functionality

pub mod sync; // Types used by the database drivers in order to sync data to the backend driver

pub use crate::luro::{
    luro_channel::LuroChannel, luro_character::LuroCharacter, luro_character::LuroCharacterFetish,
    luro_character_fetish::LuroCharacterFetishCategory, luro_character_image::LuroCharacterImage, luro_guild::LuroGuild,
    luro_guild_alert_channels::GuildAlertChannels, luro_guild_data::LuroGuildData, luro_image::LuroImage, luro_member::LuroMember,
    luro_role::LuroRole, luro_user::LuroUser, luro_user_data::LuroUserData, luro_user_type::LuroUserType, luro_word_count::WordCount,
};

#[cfg(feature = "diesel-driver")]
pub use crate::diesel::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};
#[cfg(feature = "sqlx-driver")]
pub use crate::sqlx::{
    application::{DbApplication, DbApplicationType},
    channel::{DbChannel, DbChannelType},
    interaction::{DatabaseInteraction, DatabaseInteractionKind},
    luro_database::LuroDatabase,
    member::{DbMember, DbMemberType},
    message::{DatabaseMessage, DatabaseMessageSource, DatabaseMessageType},
    role::{DbRole, DbRoleType},
    user::{DatabaseUser, DatabaseUserType, LuroUserPermissions},
    user_marriage::DbUserMarriage,
    user_marriage_approvers::{DbUserMarriageApprovals, DbUserMarriageApprovalsCount},
};
#[cfg(feature = "toml-driver")]
pub use crate::toml::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};
