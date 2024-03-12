use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(
    twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand,
)]
#[command(name = "decode", desc = "Convert a string from base64")]
pub struct Decode {
    /// Decode this string from base64
    #[command(max_length = 2039)]
    pub string: String,
}

impl crate::models::CreateCommand for Decode {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        let response =
            super::decode_response(framework.accent_colour().await, &super::decode(&self.string)?)?;
        framework.response_send(&response).await
    }
}
