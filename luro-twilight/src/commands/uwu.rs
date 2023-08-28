use luro_framework::{command::LuroCommand, Framework, InteractionCommand, LuroInteraction};
use twilight_interactions::command::{CommandModel, CreateCommand};
use uwuifier::uwuify_str_sse;

use luro_model::database::drivers::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand)]
#[command(name = "uwu", desc = "UwUify a message")]
pub struct UwU {
    /// What should I UwUify?
    message: String
}

impl LuroCommand for UwU {
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let uwu = if cfg!(target_feature = "sse4.1") {
            unsafe { sse_uwu(&self.message) }
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
