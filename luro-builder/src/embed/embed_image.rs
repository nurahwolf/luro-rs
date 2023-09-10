use twilight_model::channel::message::embed::EmbedImage;

pub struct EmbedImageBuilder(EmbedImage,);

impl EmbedImageBuilder {
    /// Set the width of the image.
    pub fn width(&mut self, width: u64,) -> &mut Self {
        self.0.width = Some(width,);
        self
    }

    /// Set the height of the image.
    pub fn height(&mut self, height: u64,) -> &mut Self {
        self.0.height = Some(height,);
        self
    }

    /// Set the icon of the footer. This should be set!
    pub fn url<S: ToString,>(&mut self, url: S,) -> &mut Self {
        self.0.url = url.to_string();
        self
    }

    /// Set the proxy icon url for the footer
    pub fn proxy_url<S: ToString,>(&mut self, url: S,) -> &mut Self {
        self.0.proxy_url = Some(url.to_string(),);
        self
    }
}

impl Default for EmbedImageBuilder {
    fn default() -> Self {
        Self(EmbedImage {
            height: None,
            proxy_url: None,
            url: "https://discord.gg".to_owned(),
            width: None,
        },)
    }
}

impl From<EmbedImageBuilder,> for EmbedImage {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: EmbedImageBuilder,) -> Self {
        builder.0
    }
}
