use twilight_model::channel::message::Component;

use crate::builders::ComponentBuilder;

use super::InteractionResponse;

impl InteractionResponse {
    pub fn components<F>(&mut self, components: F) -> &mut Self
    where
        F: FnOnce(&mut ComponentBuilder) -> &mut ComponentBuilder,
    {
        let mut c = ComponentBuilder::default();
        components(&mut c);
        match &mut self.components {
            Some(components) => components.push(c.into()),
            None => self.components = Some(c.into()),
        };
        self
    }

    pub fn add_components(&mut self, components: impl Into<Vec<Component>>) -> &mut Self {
        match &mut self.components {
            Some(existing) => existing.append(&mut components.into()),
            None => self.components = Some(components.into()),
        };
        self
    }
}
