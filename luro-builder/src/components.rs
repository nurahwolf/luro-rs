use twilight_model::channel::message::Component;

use self::{action_row::ActionRowBuilder, select_menu::SelectMenuBuilder, text_input::TextInputBuilder};

pub mod action_row;
pub mod button;
pub mod select_menu;
pub mod text_input;

#[derive(Default)]
pub struct ComponentBuilder(Vec<Component>);

impl ComponentBuilder {
    /// Create and add an action row
    pub fn action_row<F>(&mut self, action_row: F) -> &mut Self
    where
        F: FnOnce(&mut ActionRowBuilder) -> &mut ActionRowBuilder
    {
        let mut a = ActionRowBuilder::default();
        action_row(&mut a);
        self.0.push(a.into());
        self
    }

    /// Create and add an selection menu
    pub fn select_menu<F>(&mut self, select_menu: F) -> &mut Self
    where
        F: FnOnce(&mut SelectMenuBuilder) -> &mut SelectMenuBuilder
    {
        let mut s = SelectMenuBuilder::default();
        select_menu(&mut s);
        self.0.push(s.into());
        self
    }

    /// Create and add a text input
    pub fn text_input<F>(&mut self, text_input: F) -> &mut Self
    where
        F: FnOnce(&mut TextInputBuilder) -> &mut TextInputBuilder
    {
        let mut t = TextInputBuilder::default();
        text_input(&mut t);
        self.0.push(t.into());
        self
    }
}

impl From<ComponentBuilder> for Vec<Component> {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: ComponentBuilder) -> Self {
        builder.0
    }
}

impl From<ComponentBuilder> for Component {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: ComponentBuilder) -> Self {
        builder.0.last().unwrap().clone()
    }
}
