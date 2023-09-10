use super::LuroResponse;

impl LuroResponse {
    /// Set the custom ID
    pub fn custom_id(&mut self, id: impl ToString,) -> &mut Self {
        self.custom_id = Some(id.to_string(),);
        self
    }
}
