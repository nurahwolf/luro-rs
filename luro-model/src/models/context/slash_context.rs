use crate::guild::Guild;

/// A context type for slash command interactions
pub struct SlashContext<'a> {
    pub guild: Option<Guild<'a>>,
}
