use twilight_model::application::interaction::InteractionData;

impl super::InteractionContext {
    pub fn command_name(&self) -> &str {
        match &self.interaction.data {
            Some(cmd_data) => match cmd_data {
                InteractionData::ApplicationCommand(cmd) => &cmd.name,
                InteractionData::MessageComponent(cmd) => &cmd.custom_id,
                InteractionData::ModalSubmit(cmd) => &cmd.custom_id,
                _ => "unknown command",
            },
            None => self.interaction.kind.kind(),
        }
    }
}
