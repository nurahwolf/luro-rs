use twilight_model::channel::message::embed::EmbedProvider;
pub struct EmbedProviderBuilder(EmbedProvider);

impl EmbedProviderBuilder {
    /// Set the URL of the provider
    pub fn url<S: ToString>(&mut self, icon_url: S) -> &mut Self {
        self.0.url = Some(icon_url.to_string());
        self
    }

    /// Set the name of the provider
    pub fn name<S: ToString>(&mut self, name: S) -> &mut EmbedProviderBuilder {
        self.0.name = Some(name.to_string());
        self
    }
}

impl Default for EmbedProviderBuilder {
    fn default() -> Self {
        Self(EmbedProvider {
            name: None,
            url: None,
        })
    }
}

impl From<EmbedProviderBuilder> for EmbedProvider {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: EmbedProviderBuilder) -> Self {
        builder.0
    }
}
