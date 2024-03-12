impl super::MemberContext {
    /// Get's the user's username name
    ///
    /// Returns the first match
    /// Username -> Legacy Username
    pub fn username(&self) -> String {

        match self.user.discriminator == 0 {
            true => self.user.name.clone(),
            false => format!("{}#{}", self.user.name, self.user.discriminator),
        }
    }
}