use luro_framework::{CommandInteraction, CreateLuroCommand, ExecuteLuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use uwuifier::uwuify_str_sse;

#[derive(CommandModel, CreateCommand)]
#[command(name = "uwu", desc = "UwUify a message")]
pub struct UwU {
    /// What should I UwUify?
    message: String,
}

impl CreateLuroCommand for UwU {}

impl ExecuteLuroCommand for UwU {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let uwu = if cfg!(target_feature = "sse4.1") {
            unsafe { sse_uwu(&self.message) }
        } else {
            arm_uwu()
        };

        ctx.respond(|r| r.content(uwu)).await
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn sse_uwu(message: &str) -> String {
    uwuify_str_sse(message)
}

fn arm_uwu() -> String {
    "Sorry, UwU is not possible on ARM currently :(".to_owned()
}
