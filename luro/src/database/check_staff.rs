use twilight_model::id::{marker::UserMarker, Id};

use crate::models::{interaction::InteractionResult, User};

impl super::Database {
    pub async fn check_staff(&self, user_id: Id<UserMarker>) -> InteractionResult<Option<User>> {
        let staff = self.fetch_staff().await?;

        Ok(staff
            .iter()
            .find(|staff| staff.user_id() == user_id)
            .cloned())
    }
}
