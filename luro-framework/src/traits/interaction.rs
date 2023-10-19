use luro_model::builders::EmbedBuilder;
use twilight_model::{user::User, id::{marker::UserMarker, Id}};

pub trait InteractionTrait {
    fn command_name(&self) -> &str;
    fn accent_colour(&self) -> impl std::future::Future<Output = u32> + Send;
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
    fn default_embed(&self) -> impl std::future::Future<Output = EmbedBuilder> + Send
    where
        Self: Sync,
    {
        async {
            let mut embed = EmbedBuilder::default();
            embed.colour(self.accent_colour().await);
            embed
        }
    }
}