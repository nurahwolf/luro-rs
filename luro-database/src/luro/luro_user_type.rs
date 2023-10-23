use serde::{Deserialize, Serialize};

/// An Enum used to tell how a [LuroUser] was created. Additionally it wraps the type that created it.
///
/// There is also an implementation for turning this type into a [LuroUser]!
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LuroUserType {
    /// A type instanced from a Twilight [User]
    User,
    /// A type instanced from a Twilight [Member]
    Member,
    /// A type instanced from our database
    DbUser,
    /// A type instanced from our database, with guild information available
    DbMember,
}

impl std::fmt::Display for LuroUserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuroUserType::User => write!(f, "Twilight User"),
            LuroUserType::Member => write!(f, "Twilight Member"),
            LuroUserType::DbUser => write!(f, "Database User"),
            LuroUserType::DbMember => write!(f, "Database Member"),
        }
    }
}
