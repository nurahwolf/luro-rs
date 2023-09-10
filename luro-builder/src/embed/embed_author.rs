use twilight_model::channel::message::embed::EmbedAuthor;

pub struct EmbedAuthorBuilder(EmbedAuthor,);

impl EmbedAuthorBuilder {
    /// Create a new author with the required parameters
    pub fn new<S: ToString,>(name: S,) -> EmbedAuthorBuilder {
        Self(EmbedAuthor {
            icon_url: None,
            name: name.to_string(),
            proxy_icon_url: None,
            url: None,
        },)
    }

    /// Set the URL of the author's icon.
    pub fn icon_url<S: ToString,>(&mut self, icon_url: S,) -> &mut Self {
        self.0.icon_url = Some(icon_url.to_string(),);
        self
    }

    /// Set the author's name.
    pub fn name<S: ToString,>(&mut self, name: S,) -> &mut EmbedAuthorBuilder {
        self.0.name = name.to_string();
        self
    }

    /// Set the author's URL.
    pub fn url<S: ToString,>(&mut self, url: S,) -> &mut Self {
        self.0.url = Some(url.to_string(),);
        self
    }

    /// Set the author's Proxy iron URL.
    pub fn proxy_url<S: ToString,>(&mut self, url: S,) -> &mut Self {
        self.0.proxy_icon_url = Some(url.to_string(),);
        self
    }
}

impl Default for EmbedAuthorBuilder {
    fn default() -> Self {
        Self(EmbedAuthor {
            icon_url: None,
            name: "".to_owned(),
            proxy_icon_url: None,
            url: None,
        },)
    }
}

impl From<EmbedAuthorBuilder,> for EmbedAuthor {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: EmbedAuthorBuilder,) -> Self {
        builder.0
    }
}
