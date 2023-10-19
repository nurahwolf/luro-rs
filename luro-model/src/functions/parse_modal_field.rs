use anyhow::anyhow;
use twilight_model::application::interaction::modal::ModalInteractionData;

/// Parse a field from [`ModalInteractionData`].
///
/// This function try to find a field with the given name in the modal data and
/// return its value as a string.
pub fn parse_modal_field<'a>(data: &'a ModalInteractionData, name: &str) -> Result<Option<&'a str>, anyhow::Error> {
    let mut components = data.components.iter().flat_map(|c| &c.components);

    match components.find(|c| &*c.custom_id == name) {
        Some(component) => Ok(component.value.as_deref()),
        None => Err(anyhow!("missing modal field: {}", name)),
    }
}

/// Parse a required field from [`ModalInteractionData`].
///
/// This function is the same as [`parse_modal_field`] but returns an error if
/// the field value is [`None`].
pub fn parse_modal_field_required<'a>(data: &'a ModalInteractionData, name: &str) -> Result<&'a str, anyhow::Error> {
    let value = parse_modal_field(data, name)?;

    value.ok_or_else(|| anyhow!("required modal field is empty: {}", name))
}