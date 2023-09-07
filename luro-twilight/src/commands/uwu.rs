use std::sync::Arc;

use async_trait::async_trait;
use luro_framework::{
    command::{LuroCommandBuilder, LuroCommandTrait},
    Framework, InteractionCommand, LuroInteraction
};
use twilight_interactions::command::{CommandModel, CreateCommand};
use uwuifier::uwuify_str_sse;

use luro_model::database::drivers::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand)]
#[command(name = "uwu", desc = "UwUify a message")]
pub struct UwU {
    /// What should I UwUify?
    message: String
}

impl LuroCommandBuilder for UwU {}

#[async_trait]
impl LuroCommandTrait for UwU {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Arc<Framework<D>>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let uwu = if cfg!(target_feature = "sse4.1") {
            unsafe { sse_uwu(&data.message) }
        } else {
            arm_uwu()
        };

        interaction.respond(&ctx, |r| r.content(uwu)).await
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn sse_uwu(message: &str) -> String {
    uwuify_str_sse(message)
}

fn arm_uwu() -> String {
    "Sorry, UwU is not possible on ARM currently :(".to_owned()
}
