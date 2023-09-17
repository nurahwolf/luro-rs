use crate::CommandInteraction;

impl<T> CommandInteraction<T> {
    // fn command_name(&self) -> &str {
    //     match self.data {
    //         Some(ref data) => match data {
    //             InteractionData::ApplicationCommand(data) => &data.name,
    //             InteractionData::MessageComponent(data) => &data.custom_id,
    //             InteractionData::ModalSubmit(data) => &data.custom_id,
    //             _ => "unknown interaction type",
    //         },
    //         None => "ping interaction",
    //     }
    // }

    pub fn command_name(&self) -> &str {
        &self.data.name
    }
}
