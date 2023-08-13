use twilight_model::http::attachment::Attachment;

use super::LuroResponse;

impl LuroResponse {
    /// Set the content that should be sent with the message.
    /// This will overrwrite anything previously set.
    pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
        let mut content = content.into();

        #[cfg(feature = "auto-trim")]
        if content.len() > 2000 {
            let mut attachment = Attachment::from_bytes("message-content.txt".to_owned(), content.as_bytes().to_vec(), 1);
            attachment.description("The message content was too long and truncated.".to_owned());

            match &mut self.attachments {
                Some(attachments) => attachments.push(attachment),
                None => self.attachments = Some(vec![attachment])
            }
            content.truncate(1997);
            content.push_str("...");
        }

        self.content = Some(content);
        self
    }
}
