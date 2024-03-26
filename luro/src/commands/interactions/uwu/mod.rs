use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(CommandModel, CreateCommand)]
#[command(name = "uwu", desc = "UwUify a message")]
pub struct UwU {
    /// What should I UwUify?
    _message: String,
}

impl crate::models::CreateCommand for UwU {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        #[cfg(target_feature = "sse4.1")]
        let uwu = uwuifier::uwuify_str_sse(&self._message);

        #[cfg(not(target_feature = "sse4.1"))]
        let uwu = "Sorry, UwU is not possible on ARM currently :(";

        framework.respond(|r| r.content(uwu)).await
    }
}
