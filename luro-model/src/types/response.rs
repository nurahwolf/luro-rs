use crate::message::Message;

/// Luro's response type
#[derive(Debug, Default)]
pub struct CommandResponse {
    pub message: Option<Message>,
}

impl CommandResponse {
    pub fn from_message(message: Option<Message>) -> Self {
        Self { message }
    }
}

impl From<()> for CommandResponse {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<CommandResponse> for () {
    fn from(_: CommandResponse) -> Self {}
}
