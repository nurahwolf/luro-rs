use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

use super::{UserPermissions, Gender, Sexuality};

/// A type that is only present if additional details was requested, or the instance of [LuroUser] was received from the database.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UserData {
    pub gender: Option<Gender>,
    pub permissions: UserPermissions,
    pub sexuality: Option<Sexuality>,
    pub user_id: Id<UserMarker>,
}
