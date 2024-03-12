impl super::MemberContext {
    /// Returns the user's highest role. Returns none if there are no roles
    pub fn highest_role(&mut self) -> Option<&crate::models::Role> {
        self.roles.sort();
        self.roles.first()
    }

    /// Returns the user's highest role that has a colour set. Returns none if there are no roles / no colours
    pub fn highest_role_colour(&mut self) -> Option<&crate::models::Role> {
        self.roles.sort();
        self.roles.iter().find(|role| role.role.color != 0)
    }
}
