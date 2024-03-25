use crate::guild::Guild;

/// A context type for slash command interactions
pub struct SlashContext {
    pub guild: Option<Guild>,
}
