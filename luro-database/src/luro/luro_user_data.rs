use serde::{Deserialize, Serialize};

use crate::{Gender, LuroUserPermissions, Sexuality};

/// A type that is only present if additional details was requested, or the instance of [LuroUser] was received from the database.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LuroUserData {
    pub permissions: LuroUserPermissions,
    pub gender: Option<Gender>,
    pub sexuality: Option<Sexuality>,
}
