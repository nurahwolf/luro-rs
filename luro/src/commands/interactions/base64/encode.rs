use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(
    twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand,
)]
#[command(name = "encode", desc = "Convert a string to base64")]
pub struct Encode {
    /// Encode this string to base64
    pub string: String,
    /// Set to true if you want to call out someone for clicking decoding this
    pub bait: Option<bool>,
}

impl crate::models::CreateCommand for Encode {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        let response =
            super::encode_response(framework.accent_colour().await, &super::encode(&self.string))?;
        framework.response_send(&response).await
    }
}
