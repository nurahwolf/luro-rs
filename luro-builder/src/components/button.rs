use twilight_model::channel::message::{
    component::{Button, ButtonStyle},
    Component, ReactionType
};

/// A nicer way to create fields. By default you should just use the 'field' function, however you can also use the split function calls if it is neater for your use case
pub struct ButtonBuilder(Button);

impl ButtonBuilder {
    /// Set of the field should be inline or not
    pub fn custom_id(&mut self, custom_id: impl ToString) -> &mut Self {
        self.0.custom_id = Some(custom_id.to_string());
        self
    }

    /// The name of this field
    pub fn disabled(&mut self, disabled: bool) -> &mut Self {
        self.0.disabled = disabled;
        self
    }

    /// The value of this field. Note that it can only be 1024 characters long!
    pub fn emoji(&mut self, emoji: ReactionType) -> &mut Self {
        self.0.emoji = Some(emoji);
        self
    }

    /// Set of the field should be inline or not
    pub fn label(&mut self, label: impl ToString) -> &mut Self {
        self.0.label = Some(label.to_string());
        self
    }

    /// The value of this field. Note that it can only be 1024 characters long!
    pub fn style(&mut self, style: ButtonStyle) -> &mut Self {
        self.0.style = style;
        self
    }

    /// Set of the field should be inline or not
    pub fn url(&mut self, url: impl ToString) -> &mut Self {
        self.0.url = Some(url.to_string());
        self
    }
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        Self(Button {
            custom_id: None,
            disabled: false,
            emoji: None,
            label: None,
            style: ButtonStyle::Primary,
            url: None
        })
    }
}

impl From<ButtonBuilder> for Button {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: ButtonBuilder) -> Self {
        builder.0
    }
}

impl From<ButtonBuilder> for Vec<Component> {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: ButtonBuilder) -> Self {
        vec![Component::Button(builder.0)]
    }
}

impl From<ButtonBuilder> for Component {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: ButtonBuilder) -> Self {
        Component::Button(builder.0)
    }
}
