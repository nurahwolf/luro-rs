use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello")]
pub struct HelloCommand {}

#[async_trait]
impl LuroCommand for HelloCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let (_, slash_author) = ctx.get_interaction_author(&slash)?;

        let content = format!(
            "Hello World! I am **{}**. It's nice to meet you, <@{}>!",
            slash_author.name, slash_author.user_id
        );

        slash.content(content);
        ctx.respond(&mut slash).await
    }
}
