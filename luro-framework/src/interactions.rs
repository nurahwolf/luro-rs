use luro_builder::embed::EmbedBuilder;
use twilight_model::{id::{Id, marker::UserMarker}, user::User};

mod command;
mod component;
mod modal;

pub trait InteractionTrait {
    fn command_name(&self) -> &str;
    async fn accent_colour(&self) -> u32;
    fn author(&self) -> &User;

    /// ID of the user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`]'s ID and then, if not present, check the
    /// [`user`]'s ID.
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    fn author_id(&self) -> Id<UserMarker> {
        self.author().id
    }
    
    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    async fn default_embed(&self) -> EmbedBuilder {
        let mut embed = EmbedBuilder::default();
        embed.colour(self.accent_colour().await);
        embed
    }
}