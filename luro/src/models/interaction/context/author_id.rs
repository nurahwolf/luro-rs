use twilight_model::id::{marker::UserMarker, Id};

use crate::models::interaction::{InteractionError, InteractionResult};

impl super::InteractionContext {
    pub fn author_id(&self) -> InteractionResult<Id<UserMarker>> {
        match self.interaction.author_id() {
            Some(author_id) => Ok(author_id),
            None => Err(InteractionError::NoAuthor),
        }
    }
}
