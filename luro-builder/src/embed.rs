use twilight_model::{
    channel::message::{
        embed::{EmbedAuthor, EmbedField, EmbedImage, EmbedProvider, EmbedThumbnail, EmbedVideo},
        Embed
    },
    util::Timestamp
};

use self::{
    embed_author::EmbedAuthorBuilder, embed_field::EmbedFieldBuilder, embed_footer::EmbedFooterBuilder,
    embed_image::EmbedImageBuilder, embed_provider::EmbedProviderBuilder, embed_thumbnail::EmbedThumbnailBuilder,
    embed_video::EmbedVideoBuilder
};

pub mod embed_author;
pub mod embed_field;
pub mod embed_footer;
pub mod embed_image;
pub mod embed_provider;
pub mod embed_thumbnail;
pub mod embed_video;

/// Based on Serenity's builders, but hopefully engineered for more correctness. This uses function builders to make creating embeds a little more erganomic, instead of the typical route of chaining methods together. Handy for short and simple commands. This is simply a wrapper around Twilight's ['Embed'], so you can consume these in exactly the same way without needing to run `.build()` or anything. You can also turn ['EmbedAuthorBuilder'] from `twilight-util` into these types as well!
///
/// Example:
/// ```rust
/// ctx.respond(|response| {
///     response.embed(|embed| {
///         embed.title("Hello World")
///             .description("I really like you!")
///             .color(0xDABEEF)
///         })
///     }).await;
/// ```

#[derive(Clone)]
pub struct EmbedBuilder(pub Embed);

impl Default for EmbedBuilder {
    fn default() -> Self {
        // NOTE: https://discord.com/developers/docs/resources/channel#embed-object-embed-types
        // Kind (type) may be removed in the future
        Self(Embed {
            author: None,
            color: None,
            description: None,
            fields: Default::default(),
            footer: None,
            image: None,
            kind: "rich".to_owned(),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None
        })
    }
}

impl From<EmbedBuilder> for Embed {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: EmbedBuilder) -> Self {
        builder.0
    }
}

impl EmbedBuilder {
    /// Build the author of the embed. You must set name which is enforced by this builder.
    ///
    /// Refer to the documentation for [`EmbedAuthorBuilder`] for more
    /// information.
    pub fn author<F>(&mut self, author: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedAuthorBuilder) -> &mut EmbedAuthorBuilder
    {
        let mut a = EmbedAuthorBuilder::default();
        author(&mut a);
        self.0.author = Some(a.into());
        self
    }

    /// Set the author of the embed from an existing builder.
    pub fn set_author(&mut self, author: impl Into<EmbedAuthor>) -> &mut Self {
        self.0.author = Some(author.into());
        self
    }

    /// Sets the colour of an embed.
    pub fn colour<C: Into<u32>>(&mut self, colour: C) -> &mut Self {
        self.0.color = Some(colour.into());
        self
    }

    /// Sets the description of an embed.
    pub fn description<S: ToString>(&mut self, description: S) -> &mut Self {
        self.0.description = Some(description.to_string());
        self
    }

    /// Adds another field to the embed, keeping the previous ones
    /// NOTE: If the resulting embed is being sent by Luro, it is checked to make sure we are not over 25 fields.
    /// There is NO check for this in the builder itself!
    pub fn field<F>(&mut self, field: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedFieldBuilder) -> &mut EmbedFieldBuilder
    {
        let mut f = EmbedFieldBuilder::default();
        field(&mut f);
        self.0.fields.push(f.into());
        self
    }

    /// Simply a shorthand to the field function, just directly sets all three fields. Generally the most commonly used one
    /// NOTE: If the resulting embed is being sent by Luro, it is checked to make sure we are not over 25 fields.
    /// There is NO check for this in the builder itself!
    pub fn create_field<S: ToString>(&mut self, name: S, value: S, inline: bool) -> &mut Self {
        let field = EmbedField {
            inline,
            name: name.to_string(),
            value: value.to_string()
        };

        self.0.fields.push(field);
        self
    }

    /// Explicitly set the embed's fields, overwriting all previous fields.
    /// Set to an empty vec to clear fields.
    /// Modify the nested embed of this builder directly if you want to remove / modify a specific field
    pub fn set_fields(&mut self, fields: Vec<EmbedField>) -> &mut Self {
        self.0.fields = fields;
        self
    }

    /// Build the footer of the embed. You MUST set name, otherwise it defaults to ""!
    ///
    /// Refer to the documentation for [`EmbedFooterBuilder`] for more
    /// information.
    pub fn footer<F>(&mut self, footer: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedFooterBuilder) -> &mut EmbedFooterBuilder
    {
        let mut f = EmbedFooterBuilder::default();
        footer(&mut f);
        self.0.footer = Some(f.into());
        self
    }

    /// Build the image of the embed. You MUST set name, otherwise it defaults to ""!
    ///
    /// Refer to the documentation for [`EmbedImageBuilder`] for more
    /// information.
    pub fn image<F>(&mut self, image: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedImageBuilder) -> &mut EmbedImageBuilder
    {
        let mut i = EmbedImageBuilder::default();
        image(&mut i);
        self.0.image = Some(i.into());
        self
    }

    /// Set the footer of the embed from an existing builder.
    pub fn set_image(&mut self, image: impl Into<EmbedImage>) -> &mut Self {
        self.0.image = Some(image.into());
        self
    }

    /// Sets the embed kind. This should not need to be set, and Discord may deprecated this in the future.
    /// Defaults to "rich"
    pub fn kind<S: ToString>(&mut self, kind: S) -> &mut Self {
        self.0.kind = kind.to_string();
        self
    }

    pub fn provider<F>(&mut self, provider: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedProviderBuilder) -> &mut EmbedProviderBuilder
    {
        let mut p = EmbedProviderBuilder::default();
        provider(&mut p);
        self.0.provider = Some(p.into());
        self
    }

    /// Set the provider of the embed from an existing builder.
    pub fn set_provider(&mut self, provider: impl Into<EmbedProvider>) -> &mut Self {
        self.0.provider = Some(provider.into());
        self
    }

    /// Build the image of the embed. You MUST set name, otherwise it defaults to ""!
    ///
    /// Refer to the documentation for [`EmbedImageBuilder`] for more
    /// information.
    pub fn thumbnail<F>(&mut self, thumbnail: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedThumbnailBuilder) -> &mut EmbedThumbnailBuilder
    {
        let mut t = EmbedThumbnailBuilder::default();
        thumbnail(&mut t);
        self.0.thumbnail = Some(t.into());
        self
    }

    /// Set the thumbnail of the embed from an existing builder.
    pub fn set_thumbnail(&mut self, thumbnail: impl Into<EmbedThumbnail>) -> &mut Self {
        self.0.thumbnail = Some(thumbnail.into());
        self
    }

    //         /// Build the image of the embed. You MUST set name, otherwise it defaults to ""!
    // ///
    // /// Refer to the documentation for [`EmbedImageBuilder`] for more
    // /// information.
    // pub fn timestamp<F>(&mut self, timestamp: F) -> &mut Self
    // where
    //     F: FnOnce(&mut TimestampBuilder) -> &mut TimestampBuilder
    // {
    //     let mut t = TimestampBuilder::default();
    //     timestamp(&mut t);
    //     self.0.timestamp = Some(t.into());
    //     self
    // }

    /// Set the thumbnail of the embed from an existing builder.
    pub fn set_timestamp(&mut self, timestamp: impl Into<Timestamp>) -> &mut Self {
        self.0.timestamp = Some(timestamp.into());
        self
    }

    /// Build the image of the embed. You MUST set name, otherwise it defaults to ""!
    ///
    /// Refer to the documentation for [`EmbedImageBuilder`] for more
    /// information.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.0.title = Some(title.into());
        self
    }

    /// Build the image of the embed. You MUST set name, otherwise it defaults to ""!
    ///
    /// Refer to the documentation for [`EmbedImageBuilder`] for more
    /// information.
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.0.url = Some(url.into());
        self
    }

    /// Build the image of the embed. You MUST set name, otherwise it defaults to ""!
    ///
    /// Refer to the documentation for [`EmbedImageBuilder`] for more
    /// information.
    pub fn video<F>(&mut self, video: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedVideoBuilder) -> &mut EmbedVideoBuilder
    {
        let mut v = EmbedVideoBuilder::default();
        video(&mut v);
        self.0.video = Some(v.into());
        self
    }

    /// Set the thumbnail of the embed from an existing builder.
    pub fn set_video(&mut self, video: impl Into<EmbedVideo>) -> &mut Self {
        self.0.video = Some(video.into());
        self
    }
}
