use luro_model::luro_message::LuroMessage;
use twilight_model::channel::message::Embed;

use crate::embed::EmbedBuilder;

pub struct MessageBuilder(LuroMessage);

impl MessageBuilder {
    /// Create and append an embed. Multiple calls will add multiple embeds.
    ///
    /// NOTE: This WILL fail to send if more than 10 embeds are present!
    ///
    /// Refer to the documentation for [`EmbedBuilder`] for more
    /// information.
    pub fn embed<F>(&mut self, embed: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedBuilder) -> &mut EmbedBuilder
    {
        let mut e = EmbedBuilder::default();
        embed(&mut e);
        self.0.embeds.push(e.into());

        self
    }

    /// Explicitly set and overwrite all currently set embeds.
    /// Modify the nested embeds field for more advanced controls.
    ///
    /// NOTE: This WILL fail to send if more than 10 are present!
    pub fn set_embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
        self.0.embeds = embeds;

        self
    }

    /// Set the content that should be sent with the message.
    /// This will overrwrite anything previously set.
    pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
        self.0.content = Some(content.into());

        self
    }
}
