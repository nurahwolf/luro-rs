use twilight_model::channel::message::embed::EmbedField;

/// A nicer way to create fields. By default you should just use the 'field' function, however you can also use the split function calls if it is neater for your use case
pub struct EmbedFieldBuilder(EmbedField);

impl EmbedFieldBuilder {
    /// Create this field all in one function call
    pub fn field<S: ToString>(&mut self, name: S, value: S, inline: bool) -> &mut Self {
        self.0 = EmbedField {
            inline,
            name: name.to_string(),
            value: value.to_string(),
        };
        self
    }

    /// Set of the field should be inline or not
    pub fn inline(&mut self, inline: bool) -> &mut Self {
        self.0.inline = inline;
        self
    }

    /// The name of this field
    pub fn name<S: ToString>(&mut self, name: S) -> &mut Self {
        self.0.name = name.to_string();
        self
    }

    /// The value of this field. Note that it can only be 1024 characters long!
    pub fn value<S: ToString>(&mut self, value: S) -> &mut Self {
        self.0.value = value.to_string();
        self
    }
}

impl Default for EmbedFieldBuilder {
    fn default() -> Self {
        Self(EmbedField {
            inline: false,
            name: "".to_owned(),
            value: "".to_owned(),
        })
    }
}

impl From<EmbedFieldBuilder> for EmbedField {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: EmbedFieldBuilder) -> Self {
        builder.0
    }
}
