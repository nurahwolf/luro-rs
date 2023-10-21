use luro_framework::{CommandInteraction, CreateLuroCommand, InteractionTrait};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello")]
pub struct Hello {}

impl CreateLuroCommand for Hello {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let current_user = ctx.twilight_client.current_user().await?.model().await?.name;
        ctx.respond(|r| {
            r.content(format!(
                "Hello World! I am **{}**. It's nice to meet you, <@{}>!",
                current_user,
                ctx.author.user_id()
            ))
        })
        .await
    }
}
