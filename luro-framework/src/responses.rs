use luro_builder::embed::EmbedBuilder;

pub mod user_banned;
pub mod user_kicked;
pub mod user_unbanned;
pub mod user_action;

/// A wrapper around [EmbedBuilder] to make easy standardised responses
#[derive(Default, Clone)]
pub struct StandardResponse {
    /// The internal embed, if you wish to manipulate it directly
    pub embed: EmbedBuilder
}

impl StandardResponse {
    pub fn new() -> Self {
        Self {
            embed: Default::default(),
        }
    }

    /// Return the internal embed builder
    pub fn embed(&self) -> EmbedBuilder {
        self.embed
    }

    /// Append a field to state if the response was successfully sent in a DM
    pub fn dm_sent(&mut self, success: bool) -> &mut Self {
        match success {
            true => self.embed.create_field("DM Sent", "Successful", true),
            false => self.embed.create_field("DM Sent", "Failed", true)
        };
        self
    }
    
}