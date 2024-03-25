impl super::InteractionResponseBuilder {
    /// Append some attachments
    pub fn attachments<I>(&mut self, attachment: I) -> &mut Self
    where
        I: Iterator<Item = twilight_model::http::attachment::Attachment>,
    {
        match &mut self.attachments {
            Some(existing_attachment) => existing_attachment.extend(attachment),
            None => self.attachments = Some(attachment.collect()),
        }
        self
    }
}
