use luro_framework::{ExecuteLuroCommand, CommandInteraction, InteractionTrait};

use std::str;

use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "decode", desc = "Convert a string from base64")]
pub struct Decode {
    /// Decode this string from base64
    #[command(max_length = 2039)]
    pub string: String,
}

impl ExecuteLuroCommand for Decode {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let response = super::decode_response(ctx.accent_colour().await, &super::decode(&self.string)?).await?;
        ctx.response_send(response).await?;
        Ok(())
    }
}
