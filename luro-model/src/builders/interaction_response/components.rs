impl super::InteractionResponseBuilder {
    pub fn components<F>(&mut self, components: F) -> &mut Self
    where
        F: FnOnce(&mut crate::builders::ComponentBuilder) -> &mut crate::builders::ComponentBuilder,
    {
        let mut c = crate::builders::ComponentBuilder::default();
        components(&mut c);
        match &mut self.components {
            Some(components) => components.push(c.into()),
            None => self.components = Some(c.into()),
        };
        self
    }

    pub fn add_components(
        &mut self,
        components: impl Into<Vec<twilight_model::channel::message::Component>>,
    ) -> &mut Self {
        match &mut self.components {
            Some(existing) => existing.append(&mut components.into()),
            None => self.components = Some(components.into()),
        };
        self
    }
}
