use luro_framework::{CommandInteraction, Luro, LuroCommand};
use std::fmt::Write;
use tabled::builder::Builder;
use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};

use super::{TableStyle, table_style};

#[derive(CommandModel, CreateCommand)]
#[command(name = "global", desc = "Get some stats on the bullshit someone has posted.")]
pub struct Global {
    /// A particular word to show stats for
    word: Option<String>,
    /// How many top X users should I show? I default to 10!
    limit: Option<i64>,
    /// Customise how the table looks!
    style: Option<TableStyle>
}

impl LuroCommand for Global {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let limit = self.limit.unwrap_or(10) as usize;
        let mut description = String::new();
        let mut table = Builder::new();
        let mut response = ctx.acknowledge_interaction(false).await?;
        let global_messages = ctx.database.driver.messages_count_words().await?;
        let mut user_messages = ctx.database.driver.messages_count_words_by_users().await?;

        table.set_header([
            "Total Messages I have seen",
            "Total Words Said",
            "Total Unique Words",
        ]);
        table.push_record([
            global_messages.total_messages.separate_with_commas(),
            global_messages.total_words.separate_with_commas(),
            global_messages.total_unique_words.separate_with_commas(),
        ]);
        writeln!(description, "## Global Word Stats\n```\n{}```", table_style(table, self.style.as_ref()))?;

        user_messages.sort();
        user_messages.truncate(limit);
        let mut table = Builder::new();
        table.set_header(["User", "Total Messages", "Total Unique Words", "Total Words"]);

        for user_stats in user_messages {
            table.push_record([
                ctx.fetch_user_only(user_stats.author_id.unwrap()).await?.name(),
                user_stats.total_messages.to_string(),
                user_stats.total_unique_words.to_string(),
                user_stats.total_words.to_string(),
            ]);
        }

        writeln!(description, "## User Leaderboard\n```\n{}```", table_style(table, self.style.as_ref()))?;


        // if let Some(word) = self.word {
        //     writeln!(total_words, "You wanted to see stats for the word `{word}`...")?;
        // }

        response.content(description);
        ctx.response_send(response).await?;

        Ok(())
    }
}
