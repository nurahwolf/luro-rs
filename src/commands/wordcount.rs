use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use super::LuroCommand;
use crate::{models::UserData, responses::LuroSlash};
use std::fmt::Write;

#[derive(CommandModel, CreateCommand)]
#[command(name = "wordcount", desc = "Get some stats on the bullshit someone has posted.")]
pub struct WordcountCommand {
    /// The user to get the stats of
    user: ResolvedUser
}

#[async_trait]
impl LuroCommand for WordcountCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut content = "**Word Totals**\n".to_owned();
        let user_data = UserData::get_user_settings(&ctx.luro, &self.user.resolved.id).await?;

        for (word, amount) in user_data.wordcount.iter() {
            writeln!(content, "`{word}`: `{amount}`")?
        }

        ctx.content(content).respond().await
    }
}
