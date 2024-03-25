use twilight_model::id::{marker::UserMarker, Id};

use crate::{database::Error, user::UserContext};

impl crate::database::Database {
    /// Check if a user is staff. Returns the user if matches, otherwise returns none on no match
    pub async fn check_staff(&self, user_id: Id<UserMarker>) -> Result<Option<UserContext>, Error> {
        let staff = self.fetch_staff().await?;

        Ok(staff.into_iter().find(|staff| staff.twilight_user.id == user_id))
    }
}
