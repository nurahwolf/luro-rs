use twilight_model::http::attachment::Attachment;

use super::LuroResponse;

impl LuroResponse {
    /// Set the content that should be sent with the message.
    /// This will overrwrite anything previously set.
    /// Use `check_content()` if you want to append content
    pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
        let content = content.into();
        self.content = Some(content);

        #[cfg(feature = "auto-trim")]
        return self.check_content();

        #[cfg(not(feature = "auto-trim"))]
        self
    }

    #[cfg(feature = "auto-trim")]
    pub fn check_content(&mut self) -> &mut Self {
        use luro_model::ACCENT_COLOUR;

        if let Some(mut content) = self.content.clone() {
            // Can we convert this to an embed?
            if content.len() > 2000 && content.len() < 4096 {
                self.content = None;
                self.embed(|embed| embed.colour(ACCENT_COLOUR).description(content));
            } else if content.len() > 4096 {
                let mut attachment = Attachment::from_bytes("message-content.txt".to_owned(), content.as_bytes().to_vec(), 1);
                attachment.description("The message content was too long and truncated.".to_owned());

                match &mut self.attachments {
                    Some(attachments) => attachments.push(attachment),
                    None => self.attachments = Some(vec![attachment])
                }
                content.truncate(1997);
                content.push_str("...");
            }
        }
        self
    }
}
