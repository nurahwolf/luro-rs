use twilight_model::channel::message::{
    component::{TextInput, TextInputStyle},
    Component,
};

/// A nicer way to create fields. By default you should just use the 'field' function, however you can also use the split function calls if it is neater for your use case
pub struct TextInputBuilder(pub TextInput,);

impl TextInputBuilder {
    /// Developer defined identifier.
    pub fn custom_id(&mut self, custom_id: impl ToString,) -> &mut Self {
        self.0.custom_id = custom_id.to_string();
        self
    }

    pub fn label(&mut self, label: impl ToString,) -> &mut Self {
        self.0.label = label.to_string();
        self
    }

    pub fn max_length(&mut self, max_length: u16,) -> &mut Self {
        self.0.max_length = Some(max_length,);
        self
    }

    pub fn min_length(&mut self, min_length: u16,) -> &mut Self {
        self.0.min_length = Some(min_length,);
        self
    }

    pub fn required(&mut self, required: bool,) -> &mut Self {
        self.0.required = Some(required,);
        self
    }

    pub fn style(&mut self, style: TextInputStyle,) -> &mut Self {
        self.0.style = style;
        self
    }

    /// MCustom placeholder text if no option is selected.
    pub fn placeholder(&mut self, placeholder: impl ToString,) -> &mut Self {
        self.0.placeholder = Some(placeholder.to_string(),);
        self
    }

    pub fn value(&mut self, value: impl ToString,) -> &mut Self {
        self.0.value = Some(value.to_string(),);
        self
    }
}

impl Default for TextInputBuilder {
    fn default() -> Self {
        Self(TextInput {
            custom_id: "null".to_owned(),
            label: Default::default(),
            max_length: Default::default(),
            min_length: Default::default(),
            placeholder: Default::default(),
            required: Default::default(),
            style: TextInputStyle::Paragraph,
            value: Default::default(),
        },)
    }
}

impl From<TextInputBuilder,> for Component {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: TextInputBuilder,) -> Self {
        Component::TextInput(builder.0,)
    }
}
