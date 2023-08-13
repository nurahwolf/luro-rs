use twilight_model::channel::message::Component;

use self::action_row::ActionRowBuilder;

mod action_row;
mod button;

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
