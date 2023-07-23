use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::responses::LuroSlash;

use super::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello")]
pub struct HelloCommand {}

#[async_trait]
impl LuroCommand for HelloCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // TODO: Absolutely trash error handling lol
        let content = match ctx.interaction.author_id() {
            Some(author_id) => format!(
                "Hello World! I am **{}**. It's nice to meet you, <@{}>!",
                ctx.author()?.name,
                author_id
            ),
            None => format!(
                "Hello World! I am **{}**. It's nice to meet you, but unfortunately I cannot see your name :(",
                ctx.luro.twilight_client.current_user().await?.model().await?.name
            )
        };

        ctx.content(content).respond().await
    }
}
