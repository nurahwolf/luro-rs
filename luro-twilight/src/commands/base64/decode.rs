use async_trait::async_trait;
use luro_framework::{
    command::{LuroCommandBuilder, LuroCommandTrait},
    Framework, InteractionCommand, LuroInteraction,
};
use luro_model::database_driver::LuroDatabaseDriver;

use std::str;

use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "decode", desc = "Convert a string from base64")]
pub struct Decode {
    /// Decode this string from base64
    #[command(max_length = 2039)]
    pub string: String,
}

impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for Decode {}

#[async_trait]
impl LuroCommandTrait for Decode {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let response = super::decode_response(&ctx, &interaction, &super::decode(&data.string)?).await?;
        interaction.send_response(&ctx, response).await?;
        Ok(())
    }
}
