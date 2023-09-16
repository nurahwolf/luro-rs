use async_trait::async_trait;
use luro_framework::{
    command::{LuroCommandBuilder, LuroCommandTrait},
    Framework, InteractionCommand, LuroInteraction,
};
use luro_model::database_driver::LuroDatabaseDriver;

use std::str;

use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "encode", desc = "Convert a string to base64")]
pub struct Encode {
    /// Encode this string to base64
    pub string: String,
    /// Set to true if you want to call out someone for clicking decoding this
    pub bait: Option<bool>,
}

impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for Encode {}

#[async_trait]
impl LuroCommandTrait for Encode {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let response = super::encode_response(&ctx, &interaction, &super::encode(&data.string)).await?;
        interaction.send_response(&ctx, response).await?;
        Ok(())
    }
}
