use twilight_model::channel::{
    message::{
        component::{SelectMenu, SelectMenuOption, SelectMenuType},
        Component,
    },
    ChannelType,
};

/// A nicer way to create fields. By default you should just use the 'field' function, however you can also use the split function calls if it is neater for your use case
pub struct SelectMenuBuilder(SelectMenu);

impl SelectMenuBuilder {
    /// An optional list of channel types.
    ///
    /// This is only applicable to [channel select menus](SelectMenuType::Channel).
    pub fn channel_types(&mut self, mut channel_types: Vec<ChannelType>) -> &mut Self {
        match &mut self.0.channel_types {
            Some(types) => types.append(&mut channel_types),
            None => self.0.channel_types = Some(channel_types),
        }
        self
    }

    /// Developer defined identifier.
    pub fn custom_id(&mut self, custom_id: impl ToString) -> &mut Self {
        self.0.custom_id = custom_id.to_string();
        self
    }

    /// Whether the select menu is disabled.
    ///
    /// Defaults to false.
    pub fn disabled(&mut self, disabled: bool) -> &mut Self {
        self.0.disabled = disabled;
        self
    }

    /// This select menu's type.
    ///
    /// Defaults to SelectMenuType::Mentionable
    pub fn kind(&mut self, kind: SelectMenuType) -> &mut Self {
        self.0.kind = kind;
        self
    }

    /// Maximum number of options that may be chosen
    pub fn max_values(&mut self, max_values: u8) -> &mut Self {
        self.0.max_values = Some(max_values);
        self
    }

    /// Minimum number of options that may be chosen
    pub fn min_values(&mut self, min_values: u8) -> &mut Self {
        self.0.min_values = Some(min_values);
        self
    }

    /// A list of available options.
    ///
    /// This is required by [text select menus](SelectMenuType::Text).
    pub fn new_options(&mut self, mut new_options: Vec<SelectMenuOption>) -> &mut Self {
        match &mut self.0.options {
            Some(options) => options.append(&mut new_options),
            None => self.0.options = Some(new_options),
        }
        self
    }

    /// MCustom placeholder text if no option is selected.
    pub fn placeholder(&mut self, placeholder: impl ToString) -> &mut Self {
        self.0.placeholder = Some(placeholder.to_string());
        self
    }
}

impl Default for SelectMenuBuilder {
    fn default() -> Self {
        Self(SelectMenu {
            channel_types: Default::default(),
            custom_id: Default::default(),
            disabled: Default::default(),
            kind: SelectMenuType::Mentionable,
            max_values: Default::default(),
            min_values: Default::default(),
            options: Default::default(),
            placeholder: Default::default(),
        })
    }
}

impl From<SelectMenuBuilder> for Component {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: SelectMenuBuilder) -> Self {
        Component::SelectMenu(builder.0)
    }
}
