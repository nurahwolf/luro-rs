use crate::models::interaction::{InteractionError, InteractionResult};

impl super::InteractionContext {
    pub fn compontent_message(&self) -> InteractionResult<&twilight_model::channel::Message> {
        match &self.interaction.message {
            Some(message) => Ok(message),
            None => Err(InteractionError::NotComponent),
        }
    }
}
