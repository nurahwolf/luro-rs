use crate::LuroUserPermissions;

/// A type that is only present if additional details was requested, or the instance of [LuroUser] was received from the database.
#[derive(Clone, Debug)]
pub struct LuroUserData {
    pub permissions: LuroUserPermissions,
}
