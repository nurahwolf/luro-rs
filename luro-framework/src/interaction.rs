use twilight_model::{
    id::{marker::UserMarker, Id},
    user::User,
};

pub trait InteractionTrait {
    fn command_name(&self) -> &str;
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
}
