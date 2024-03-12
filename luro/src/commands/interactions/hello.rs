use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello")]
pub struct Hello {}

impl crate::models::CreateCommand for Hello {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        let current_user = framework.gateway
            .twilight_client
            .current_user()
            .await?
            .model()
            .await?
            .name;
        framework.respond(|r| {
            r.content(format!(
                "Hello World! I am **{}**. It's nice to meet you, <@{}>!",
                current_user, framework.author_id().unwrap()
            ))
        })
        .await
    }
}
