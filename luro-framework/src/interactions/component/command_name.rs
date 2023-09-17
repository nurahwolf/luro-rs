use crate::ComponentInteraction;

impl<T> ComponentInteraction<T> {
    pub fn command_name(&self) -> &str {
        &self.data.custom_id
    }
}