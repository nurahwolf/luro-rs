

use twilight_interactions::command::{CommandModel, CreateCommand};
use uwuifier::uwuify_str_sse;

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "uwu", desc = "UwUify a message")]
pub struct UwUCommand {
    /// What should I UwUify?
    message: String
}


impl LuroCommand for UwUCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let uwu = if cfg!(target_feature = "sse4.1") {
            unsafe { sse_uwu(&self.message) }
        } else {
            arm_uwu()
        };

        ctx.content(uwu).respond().await
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn sse_uwu(message: &str) -> String {
    uwuify_str_sse(message)
}

fn arm_uwu() -> String {
    "Sorry, UwU is not possible on ARM currently :(".to_owned()
}
