use twilight_model::channel::message::embed::EmbedFooter;

pub struct EmbedFooterBuilder(EmbedFooter);

impl EmbedFooterBuilder {
    /// Set the text of the footer. This should be set!
    pub fn text<S: ToString>(&mut self, text: S) -> &mut Self {
        self.0.text = text.to_string();
        self
    }

    /// Set the icon of the footer
    pub fn icon_url<S: ToString>(&mut self, url: S) -> &mut Self {
        self.0.icon_url = Some(url.to_string());
        self
    }

    /// Set the proxy icon url for the footer
    pub fn proxy_url<S: ToString>(&mut self, url: S) -> &mut Self {
        self.0.proxy_icon_url = Some(url.to_string());
        self
    }
}

impl Default for EmbedFooterBuilder {
    fn default() -> Self {
        Self(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: "".to_owned()
        })
    }
}

impl From<EmbedFooterBuilder> for EmbedFooter {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: EmbedFooterBuilder) -> Self {
        builder.0
    }
}
