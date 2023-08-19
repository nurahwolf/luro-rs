use twilight_model::channel::message::{component::ActionRow, Component};

use super::{button::ButtonBuilder, text_input::TextInputBuilder, ComponentBuilder};

pub struct ActionRowBuilder(ActionRow);

impl ActionRowBuilder {
    /// Create and add a component to this action row
    pub fn component<F>(&mut self, component: F) -> &mut Self
    where
        F: FnOnce(&mut ComponentBuilder) -> &mut ComponentBuilder
    {
        let mut c = ComponentBuilder::default();
        component(&mut c);
        self.0.components.push(c.into());
        self
    }

    /// Create and add an action row
    pub fn button<F>(&mut self, button: F) -> &mut Self
    where
        F: FnOnce(&mut ButtonBuilder) -> &mut ButtonBuilder
    {
        let mut b = ButtonBuilder::default();
        button(&mut b);
        self.0.components.push(b.into());
        self
    }

    /// Create and add a text input
    pub fn text_input<F>(&mut self, text_input: F) -> &mut Self
    where
        F: FnOnce(&mut TextInputBuilder) -> &mut TextInputBuilder
    {
        let mut t = TextInputBuilder::default();
        text_input(&mut t);
        self.0.components.push(t.into());
        self
    }
}

impl Default for ActionRowBuilder {
    fn default() -> Self {
        Self(ActionRow {
            components: Default::default()
        })
    }
}

impl From<ActionRowBuilder> for ActionRow {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: ActionRowBuilder) -> Self {
        builder.0
    }
}

impl From<ActionRowBuilder> for Component {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: ActionRowBuilder) -> Self {
        Component::ActionRow(builder.0)
    }
}

impl From<&ActionRowBuilder> for Component {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: &ActionRowBuilder) -> Self {
        Component::ActionRow(builder.0.clone())
    }
}
