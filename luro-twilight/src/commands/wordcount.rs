use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    fmt::Write,
};

use anyhow::Context;
use luro_framework::{CommandInteraction, CreateLuroCommand, InteractionTrait, LuroCommand, Luro};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    http::interaction::InteractionResponseType,
    id::{marker::UserMarker, Id},
};

use std::iter::FromIterator;

#[derive(CommandModel, CreateCommand)]
#[command(name = "wordcount", desc = "Get some stats on the bullshit someone has posted.")]
pub struct Wordcount {
    /// The user to get the stats of
    user: Option<ResolvedUser>,
    /// How many words we should get stats for. Defaults to 10.
    limit: Option<i64>,
    /// A particular word to search word
    word: Option<String>,
    /// Search across ALL user data for word stats. This can be very slow!
    global: Option<bool>,
}

impl CreateLuroCommand for Wordcount {}

impl LuroCommand for Wordcount {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        ctx.acknowledge_interaction(false).await?;
        let luro_user = ctx.get_specified_user_or_author(self.user.as_ref()).await?;
        let response = InteractionResponseType::DeferredChannelMessageWithSource;
        let accent_colour = ctx.accent_colour().await;
        let mut wordcount: usize = Default::default();
        let mut averagesize: usize = Default::default();
        let mut wordsize: BTreeMap<usize, usize> = Default::default();
        let mut words: BTreeMap<String, usize> = Default::default();
        let mut content = String::new();
        let global = self.global.unwrap_or(false);
        // How many items we should get
        let limit = match self.limit {
            Some(limit) => limit.try_into().context("Failed to convert i64 into usize")?,
            None => 10,
        };

        if global {
            let mut most_said_words: BTreeMap<Id<UserMarker>, usize> = Default::default();
            let mut user_ids = vec![];

            for (id, user_data) in data {
                user_ids.push(id);

                wordcount += user_data.wordcount;
                averagesize += user_data.averagesize;

                for (word, count) in user_data.words.clone().into_iter() {
                    *words.entry(word).or_insert(0) += count;
                    *most_said_words.entry(id).or_insert(0) += count;
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
            wordcount = luro_user.wordcount;
            averagesize = luro_user.averagesize;
            wordsize = luro_user.wordsize.clone();
            words = luro_user.words.clone();
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
                    return ctx
                        .respond(|r| {
                            r.embed(|e| {
                                e.description(content)
                                    .colour(accent_colour)
                                    .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()))
                            })
                            .response_type(response)
                        })
                        .await;
                }
                None => {
                    content = format!("The word `{word}` has never been said, as far as I can see!");
                    return ctx.respond(|r| r.content(content).response_type(response)).await;
                }
            }
        };

        // Word size field
        let mut word_size = String::new();
        let mut number_lengths = vec![];
        let mut common_word_length = Vec::from_iter(wordsize);
        common_word_length.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        common_word_length.truncate(limit);
        // First loop is for calculating total length
        for (length, total) in &common_word_length {
            // Convert to base 10
            let total = match usize::try_from(total.checked_ilog10().unwrap_or(1)) {
                Ok(total) => total + 1,
                Err(_) => continue,
            };

            let length = match usize::try_from(length.checked_ilog10().unwrap_or(1)) {
                Ok(length) => length + 1,
                Err(_) => continue,
            };

            number_lengths.push((total, length))
        }

        let padding = padding_calculator(number_lengths.clone());
        // Now loop through again, using our calculated padding
        for (length, total) in &common_word_length {
            let total_padding = padding.0;
            let length_padding = padding.1;

            writeln!(
                word_size,
                "- `{total:^total_padding$}` words with `{length:^length_padding$}` characters"
            )?;
        }

        // Most used words field
        let mut most_used = String::new();
        let mut most_used_words = Vec::from_iter(words);
        most_used_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        most_used_words.truncate(limit);

        let mut number_lengths = vec![];
        // First loop is for calculating total length
        for (word, count) in &most_used_words {
            // Convert to base 10
            let count = match usize::try_from(count.checked_ilog10().unwrap_or(1)) {
                Ok(total) => total,
                Err(_) => continue,
            };
            number_lengths.push((word.len(), count))
        }

        let padding = padding_calculator(number_lengths.clone());
        for (word, count) in &most_used_words {
            let word_padding = padding.0;
            let count_padding = padding.1;

            writeln!(
                most_used,
                "- `{word:^word_padding$}` words with `{count:^count_padding$}` characters"
            )?;
        }

        ctx.respond(|r| {
            r.embed(|embed| {
                embed
                    .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()))
                    .description(content)
                    .field(|field| field.field("Word Length", &word_size, true))
                    .field(|field| field.field("Most used words", &most_used, true))
                    .footer(|footer| footer.text(""))
                    .colour(accent_colour)
            })
            .response_type(response)
        })
        .await
    }
}

/// Work out how many padding characters is needed for a nicely formatted table.
/// This takes a vector containing the word / number lengths in base10, and provices you with the lenth
/// This is broken up by the length of the prefix, suffix and together.
pub fn padding_calculator(input: Vec<(usize, usize)>) -> (usize, usize, usize) {
    let mut prefix_length = 0;
    let mut suffix_length = 0;

    for (prefix, suffix) in input {
        if prefix > prefix_length {
            prefix_length = prefix
        }

        if suffix > suffix_length {
            suffix_length = suffix
        }
    }

    (prefix_length, suffix_length, prefix_length + suffix_length)
}
