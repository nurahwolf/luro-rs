use crate::ModalInteraction;

impl ModalInteraction {
    pub fn command_name(&self) -> &str {
        &self.data.custom_id
    }
}
