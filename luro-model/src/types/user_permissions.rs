use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserPermissions {
    Administrator,
    Owner,
    #[default]
    User,
}

impl std::fmt::Display for UserPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserPermissions::Administrator => write!(f, "ADMINISTRATOR"),
            UserPermissions::Owner => write!(f, "OWNER"),
            UserPermissions::User => write!(f, "USER"),
        }
    }
}