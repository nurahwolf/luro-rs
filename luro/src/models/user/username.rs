impl super::User {
    /// Get's the user's username name
    ///
    /// Returns the first match
    /// Username -> Legacy Username
    pub fn username(&self) -> String {
        match self {
            Self::User(user) => user.username(),
            Self::Member(member) => member.username(),
        }
    }
}

impl super::UserContext {
    /// Get's the user's username name
    ///
    /// Returns the first match
    /// Username -> Legacy Username
    pub fn username(&self) -> String {
        match self.discriminator == 0 {
            true => self.name.clone(),
            false => format!("{}#{}", self.name, self.discriminator),
        }
    }
}