use twilight_model::application::interaction::InteractionData;

use crate::models::interaction::InteractionError;

impl super::InteractionContext {
    /// Parse a field from [`ModalInteractionData`].
    ///
    /// This function try to find a field with the given name in the modal data and
    /// return its value as a string.
    pub fn parse_field(&self, name: &str) -> Result<Option<&str>, InteractionError> {
        let InteractionData::ModalSubmit(data) = self.interaction.data.as_ref().unwrap() else {
            return Err(InteractionError::NotModal);
        };

        let mut components = data.components.iter().flat_map(|c| &c.components);

        match components.find(|c| &*c.custom_id == name) {
            Some(component) => Ok(component.value.as_deref()),
            None => Err(InteractionError::MissingModalField(name.to_owned())),
        }
    }

    /// Parse a required field from [`ModalInteractionData`].
    ///
    /// This function is the same as [`parse_modal_field`] but returns an error if
    /// the field value is [`None`].
    pub fn parse_field_required(&self, name: &str) -> Result<&str, InteractionError> {
        let value = self.parse_field(name)?;

        value.ok_or_else(|| InteractionError::MissingModalField(name.to_owned()))
    }
}
