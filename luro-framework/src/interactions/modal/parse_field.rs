use crate::ModalInteraction;
use anyhow::anyhow;

impl ModalInteraction {
    /// Parse a field from [`ModalInteractionData`].
    ///
    /// This function try to find a field with the given name in the modal data and
    /// return its value as a string.
    pub fn parse_field(&self, name: &str) -> Result<Option<&str>, anyhow::Error> {
        let mut components = self.data.components.iter().flat_map(|c| &c.components);

        match components.find(|c| &*c.custom_id == name) {
            Some(component) => Ok(component.value.as_deref()),
            None => Err(anyhow!("missing modal field: {}", name)),
        }
    }

    /// Parse a required field from [`ModalInteractionData`].
    ///
    /// This function is the same as [`parse_modal_field`] but returns an error if
    /// the field value is [`None`].
    pub fn parse_field_required(&self, name: &str) -> Result<&str, anyhow::Error> {
        let value = self.parse_field(name)?;

        value.ok_or_else(|| anyhow!("required modal field is empty: {}", name))
    }
}
