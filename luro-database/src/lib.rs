// Drivers
#[cfg(feature = "diesel-driver")]
mod diesel;
#[cfg(feature = "sqlx-driver")]
mod sqlx;
#[cfg(feature = "toml-driver")]
mod toml;

mod database;
mod luro; // Functionality meant to be consumed by users of this crate. Names are prefixed with 'Luro'. // Standard Database functionality

pub use crate::luro::{
    luro_character::LuroCharacter, luro_character::LuroCharacterFetish, luro_character_fetish::LuroCharacterFetishCategory,
    luro_character_image::LuroCharacterImage, luro_image::LuroImage, luro_member::LuroMember, luro_user::LuroUser,
    luro_user_data::LuroUserData, luro_user_type::LuroUserType,
};

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
    user_marriage_approvers::{DbUserMarriageApprovals, DbUserMarriageApprovalsCount},
};
#[cfg(feature = "toml-driver")]
pub use crate::toml::{DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser, LuroDatabase};
