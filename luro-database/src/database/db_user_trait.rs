use twilight_model::id::{Id, marker::UserMarker};

/// A trait used for common user utilities, implemented for both [DbUser] and [DbMember]
pub trait DbUserTrait {
    /// Returns a [Id<UserMarker>].
    fn user_id(&self) -> Id<UserMarker>;


}