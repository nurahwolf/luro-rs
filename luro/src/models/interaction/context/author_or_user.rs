use luro_model::{database::Error, user::User};
use twilight_model::id::{marker::UserMarker, Id};

impl super::InteractionContext {
    /// Get a specified user, else fall back to the interaction author
    pub async fn author_or_user(&self, user_id: Option<Id<UserMarker>>) -> Result<User, Error> {
        match user_id {
            Some(user) => self.fetch_user(user).await,
            None => self.fetch_user(self.author_id()).await,
        }
    }
}
