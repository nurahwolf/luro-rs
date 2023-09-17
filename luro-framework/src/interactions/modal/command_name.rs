use crate::ModalInteraction;

impl<T> ModalInteraction<T> {
    pub fn command_name(&self) -> &str {
        &self.data.custom_id
    }
}
