use std::{collections::BTreeMap, convert::TryFrom};

use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::UserMarker, Id};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFieldBuilder};

use crate::{
    models::{LuroSlash, UserData},
    traits::{luro_command::LuroCommand, luro_functions::LuroFunctions}
};
use std::{convert::TryInto, fmt::Write, iter::FromIterator};

#[derive(CommandModel, CreateCommand)]
#[command(name = "wordcount", desc = "Get some stats on the bullshit someone has posted.")]
pub struct WordcountCommand {
    /// The user to get the stats of
    user: Option<ResolvedUser>,
    /// How many words we should get stats for. Defaults to 10.
    limit: Option<i64>,
    /// A particular word to search word
    word: Option<String>,
    /// Search across ALL user data for word stats. This can be very slow!
    global: Option<bool>
}

#[async_trait]
impl LuroCommand for WordcountCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let mut wordcount: usize = Default::default();
        let mut averagesize: usize = Default::default();
        let mut wordsize: BTreeMap<usize, usize> = Default::default();
        let mut words: BTreeMap<String, usize> = Default::default();
        let mut embed = ctx.default_embed().await?;
        let mut content = String::new();
        let mut digits = 0;
        let global = match self.global {
            Some(global) => {
                ctx.deferred().await?;
                global
            }
            None => false
        };
        // How many items we should get
        let limit = self
            .limit
            .unwrap_or(10)
            .try_into()
            .context("Attempted to turn the limit into a usize")?;

        if global {
            let mut most_said_words: BTreeMap<Id<UserMarker>, usize> = Default::default();
            let mut user_ids = vec![];
            for user_data in ctx.luro.user_data.iter() {
                user_ids.push(*user_data.key());

                wordcount += user_data.wordcount;
                averagesize += user_data.averagesize;

                for (word, count) in user_data.words.clone().into_iter() {
                    *words.entry(word).or_insert(0) += count;
                    *most_said_words.entry(*user_data.key()).or_insert(0) += count;
                }

                for (size, count) in user_data.wordsize.clone().into_iter() {
                    *wordsize.entry(size).or_insert(0) += count;
                }
            }

            writeln!(
                content,
                "Words counted from a total of **{}** users and I am showing stats for **{limit}** users!\n-----",
                user_ids.len()
            )?;

            let mut high_score_users = Vec::from_iter(most_said_words);
            high_score_users.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
            high_score_users.truncate(limit);

            for (user_number, (user, count)) in high_score_users.into_iter().enumerate() {
                writeln!(content, "{user_number}. <@{user}> has said `{count}` words!")?;
            }
            if content.len() > 3800 {
                content.truncate(3800);
                content.push_str("...")
            }
            writeln!(content, "-----")?;
        } else {
            let (user, slash_author) = ctx.get_specified_user_or_author(&self.user, &ctx.interaction)?;
            let author = EmbedAuthorBuilder::new(&slash_author.name).icon_url(slash_author.try_into()?);
            let user_data = UserData::get_user_settings(&ctx.luro, &user.id).await?;
            embed = embed.author(author);
            wordcount = user_data.wordcount;
            averagesize = user_data.averagesize;
            wordsize = user_data.wordsize.clone();
            words = user_data.words.clone();
        };

        let averagesize = averagesize.checked_div(wordcount).unwrap_or(0);
        writeln!(
            content,
            "Approximately **{}** words have been said with an average of **{}** letters per word.",
            wordcount, averagesize
        )?;

        // Handle if a user is just interested in a word
        if let Some(word) = self.word {
            match words.get(&word) {
                // If we are getting a single word, then we want to get it from the BTreeMap that is sorted by key
                Some(word_count) => {
                    writeln!(
                        content,
                        "-----\nSpecifically, the word `{word}` has been said about `{word_count}` times!"
                    )?;
                    return ctx.embed(embed.description(content).build())?.respond().await;
                }
                None => {
                    content = format!("The word `{word}` has never been said, as far as I can see!");
                    return ctx.content(content).respond().await;
                }
            }
        };

        // Word size field
        let mut count_size = 0;
        let mut word_size = String::new();
        for (size, count) in wordsize.iter().take(limit) {
            if let (Ok(size), Ok(count)) = (
                usize::try_from(size.checked_ilog10().unwrap_or(0) + 1),
                usize::try_from(count.checked_ilog10().unwrap_or(0) + 1)
            ) {
                if word_size.len() > 1000 {
                    break
                }

                if digits < count {
                    digits = count
                }
                if count_size < size {
                    count_size = size
                }
            }
            writeln!(
                word_size,
                "`{:^2$}` words with `{:^3$}` characters",
                count, size, digits, count_size
            )?;
        }
        word_size.truncate(1024);
        embed = embed.field(EmbedFieldBuilder::new("Word Length", word_size).inline());

        // Most used words field
        let mut most_used = String::new();
        let mut most_used_words = Vec::from_iter(words);
        most_used_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        most_used_words.truncate(limit);
        digits = 0;
        let mut word_length = 1;
        for (word, count) in most_used_words {
            if most_used.len() > 1000 {
                break
            }

            if let Ok(length) = usize::try_from(count.checked_ilog10().unwrap_or(0) + 1) {
                if digits < length {
                    digits = length
                }
            }

            if word_length < word.len() {
                word_length = word.len()
            }
            writeln!(most_used, "`{:^3$}` said `{:^2$}` times", word, count, digits, word_length)?;
        }
        most_used.truncate(1024);
        embed = embed.field(EmbedFieldBuilder::new("Most used words", most_used).inline());
        if content.len() > 4096 {
            content.truncate(4093);
            content.push_str("...")
        }
        ctx.embed(embed.description(content).build())?.respond().await
    }
}
