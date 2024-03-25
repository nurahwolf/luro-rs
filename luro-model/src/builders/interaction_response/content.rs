use super::InteractionResponseBuilder;

impl InteractionResponseBuilder {
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
        use twilight_model::http::attachment::Attachment;

        use crate::{builders::EmbedBuilder, response::safe_truncate, ACCENT_COLOUR};

        if let Some(ref mut content) = self.content {
            // Can we convert this to an embed?
            if content.len() > 2000 && content.len() < 4096 && !content.contains("```") {
                let mut embed = EmbedBuilder::default();
                embed.colour(ACCENT_COLOUR).description(content.clone());

                match &mut self.embeds {
                    Some(embeds) => embeds.push(embed.into()),
                    None => self.embeds = Some(vec![embed.into()]),
                }

                content.truncate(0);
                content.push_str("Response is a lil too big! How about an embed, instead?");
            } else if content.len() > 4096 {
                let mut attachment = Attachment::from_bytes("message-content.txt".to_owned(), content.as_bytes().to_vec(), 1);
                attachment.description("The message content was too long and truncated.".to_owned());

                match &mut self.attachments {
                    Some(attachments) => attachments.push(attachment),
                    None => self.attachments = Some(vec![attachment]),
                }

                match content.contains("```") {
                    true => {
                        safe_truncate(content, 1993);
                        content.push_str("\n```");
                    }
                    false => safe_truncate(content, 1997),
                }

                content.push_str("...");
            }
        }
        self
    }
}
