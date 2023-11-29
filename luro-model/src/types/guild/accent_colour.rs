impl super::Guild {
    /// Filter through the guild's roles to work out the guild's accent colour.
    /// Takes the highest role.
    /// 
    /// Returns: Colour, Position, Role
    pub fn accent_colour(&mut self) -> Option<(usize, &crate::types::Role)> {
        self.roles.sort();
        for (iteration, role) in self.roles.iter().enumerate() {
            if role.colour != 0 {
                return Some((iteration, role))
            }
        }

        None
    }
}