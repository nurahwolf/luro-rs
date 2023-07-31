use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFieldBuilder, ImageSource};

use super::LuroCommand;
use crate::{models::UserData, responses::LuroSlash};
use std::{convert::TryInto, fmt::Write, iter::FromIterator};

#[derive(CommandModel, CreateCommand)]
#[command(name = "wordcount", desc = "Get some stats on the bullshit someone has posted.")]
pub struct WordcountCommand {
    /// The user to get the stats of
    user: ResolvedUser,
    /// How many words we should get stats for. Defaults to 10.
    limit: Option<i64>,
    /// A particular word to search word
    word: Option<String>
}

#[async_trait]
impl LuroCommand for WordcountCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let limit = self
            .limit
            .unwrap_or(10)
            .try_into()
            .context("Attempted to turn the limit into a usize")?;
        let mut content = String::new();
        let user_data = UserData::get_user_settings(&ctx.luro, &self.user.resolved.id).await?;
        let mut embed = ctx.default_embed().await?;

        let user_name = match self.user.member {
            Some(ref member) => member.clone().nick.unwrap_or(self.user.resolved.name.clone()),
            None => self.user.resolved.name.clone()
        };
        let user_avatar =
            self.get_interaction_member_avatar(self.user.member.clone(), &ctx.interaction.guild_id, &self.user.resolved);
        let author = EmbedAuthorBuilder::new(user_name).icon_url(ImageSource::url(user_avatar)?);

        if let Some(word) = self.word {
            match user_data.words.get(&word) {
                // If we are getting a single word, then we want to get it from the BTreeMap that is sorted by key
                Some(word_count) => {
                    content = format!("**{word}:** `{word_count}`");
                    return ctx.embed(embed.description(content).author(author).build())?.respond().await;
                }
                None => {
                    return ctx
                        .content(format!(
                            "Sorry! That user has never said the word `{word}` as far as I know! :("
                        ))
                        .respond()
                        .await
                }
            }
        };

        let averagesize = user_data.averagesize / user_data.wordcount;
        writeln!(
            content,
            "The user has said **{}** words with an average of **{}** letters per word.\n",
            user_data.wordcount, averagesize
        )?;

        let mut word_size = String::new();
        for (size, count) in user_data.wordsize.iter().take(limit) {
            writeln!(word_size, "There are {} words of size {}.", count, size)?;
        }
        word_size.truncate(1024);
        embed = embed.field(EmbedFieldBuilder::new("Word Length", word_size).inline());

        let mut most_used = String::new();
        let mut most_used_words = Vec::from_iter(user_data.words);
        most_used_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        most_used_words.truncate(limit);
        for (word, count) in most_used_words {
            writeln!(most_used, "`{word}` said `{count}` times")?;
        }
        most_used.truncate(1024);
        embed = embed.field(EmbedFieldBuilder::new("Most used words", most_used).inline());

        ctx.embed(embed.description(content).author(author).build())?.respond().await
    }
}
