use crate::ComponentInteraction;

impl ComponentInteraction {
    pub fn command_name(&self) -> &str {
        &self.data.custom_id
    }
}
