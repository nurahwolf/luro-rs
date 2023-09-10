use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand,)]
#[command(name = "hello", desc = "Say hello")]
pub struct HelloCommand {}

impl LuroCommand for HelloCommand {
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        let content = match ctx.interaction.author_id() {
            Some(author_id,) => format!(
                "Hello World! I am **{}**. It's nice to meet you, <@{}>!",
                ctx.framework.twilight_client.current_user().await?.model().await?.name,
                author_id
            ),
            None => format!(
                "Hello World! I am **{}**. It's nice to meet you, but unfortunately I cannot see your name :(",
                ctx.framework.twilight_client.current_user().await?.model().await?.name
            ),
        };

        ctx.respond(|r| r.content(content,),).await
    }
}
