use super::LuroResponse;

impl LuroResponse {
    /// Set the title of a model response
    pub fn title(&mut self, title: impl ToString,) -> &mut Self {
        self.title = Some(title.to_string(),);
        self
    }
}
