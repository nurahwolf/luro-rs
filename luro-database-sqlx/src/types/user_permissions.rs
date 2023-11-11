use luro_model::types::UserPermissions;

#[derive(Debug, Default, Clone, ::sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DbUserPermissions {
    Administrator,
    Owner,
    #[default]
    User,
}

impl From<UserPermissions> for DbUserPermissions {
    fn from(permissions: UserPermissions) -> Self {
        match permissions {
            UserPermissions::Administrator => Self::Administrator,
            UserPermissions::Owner => Self::Owner,
            UserPermissions::User => Self::User,
        }
    }
}

impl From<DbUserPermissions> for UserPermissions {
    fn from(permissions: DbUserPermissions) -> Self {
        match permissions {
            DbUserPermissions::Administrator => Self::Administrator,
            DbUserPermissions::Owner => Self::Owner,
            DbUserPermissions::User => Self::User,
        }
    }
}
