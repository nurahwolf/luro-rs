use twilight_model::http::attachment::Attachment;

use super::LuroResponse;

impl LuroResponse {
    /// Append some attachments
    pub fn attachments<I>(&mut self, attachment: I) -> &mut Self
    where
        I: Iterator<Item = Attachment>,
    {
        match &mut self.attachments {
            Some(existing_attachment) => existing_attachment.extend(attachment),
            None => self.attachments = Some(attachment.collect()),
        }
        self
    }
}
