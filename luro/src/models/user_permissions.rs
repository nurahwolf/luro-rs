#[cfg(not(feature = "database-sqlx"))]

#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize, Default)]
pub enum UserPermissions {
    Administrator,
    Owner,
    #[default]
    User,
}

#[cfg(feature = "database-sqlx")]

#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize, Default, ::sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
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
