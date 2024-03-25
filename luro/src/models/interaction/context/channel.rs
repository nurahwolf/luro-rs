use twilight_model::channel::Channel;

impl super::InteractionContext {
    /// Gets the interaction channel. Panics on a ping interaction.
    pub fn channel(&self) -> &Channel {
        self.interaction.channel.as_ref().unwrap()
    }
}
