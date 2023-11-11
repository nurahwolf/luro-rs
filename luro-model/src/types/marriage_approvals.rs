use twilight_model::id::{marker::UserMarker, Id};
pub struct MarriageApprovals {
    pub user_id: Id<UserMarker>,
    pub proposer_id: Id<UserMarker>,
    pub proposee_id: Id<UserMarker>,
    pub approve: bool,
    pub disapprove: bool,
}
