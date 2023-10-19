use std::str;

use luro_framework::{LuroCommand, CommandInteraction, InteractionTrait};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "encode", desc = "Convert a string to base64")]
pub struct Encode {
    /// Encode this string to base64
    pub string: String,
    /// Set to true if you want to call out someone for clicking decoding this
    pub bait: Option<bool>,
}

impl LuroCommand for Encode {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let response = super::encode_response(ctx.accent_colour().await, &super::encode(&self.string)).await?;
        ctx.response_create(&response).await?;
        Ok(())
    }
}
