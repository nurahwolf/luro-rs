use luro_framework::{CommandInteraction, LuroCommand};
use tabled::builder::Builder;
use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

use super::{table_style, TableStyle};

#[derive(CommandModel, CreateCommand)]
#[command(name = "personal", desc = "Get some stats on the bullshit someone has posted.")]
pub struct Personal {
    /// The user to get the stats of
    user: Option<Id<UserMarker>>,
    /// A particular word to show stats for
    word: Option<String>,
    /// Customise how the table looks!
    style: Option<TableStyle>,
}

impl LuroCommand for Personal {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let user = ctx.get_specified_user_or_author(self.user).await?;
        let mut table = Builder::new();
        let mut response = ctx.acknowledge_interaction(false).await?;
        let global_messages = ctx.database.driver.messages_count_word_totals().await?;
        let user_messages = ctx.database.driver.messages_count_words_by_user(user.user_id).await?;

        table.set_header(["User", "Total Messages I have seen", "Total Words Said", "Total Unique Words"]);
        table.push_record([
            "Global",
            &global_messages.total_messages.separate_with_commas(),
            &global_messages.total_words.separate_with_commas(),
            &global_messages.total_unique_words.separate_with_commas(),
        ]);
        table.push_record([
            &user.name(),
            &user_messages.total_messages.separate_with_commas(),
            &user_messages.total_words.separate_with_commas(),
            &user_messages.total_unique_words.separate_with_commas(),
        ]);

        if let Some(word) = self.word {
            let total_times_said = match ctx.database.driver.messages_count_word_said(&word).await? {
                Some(count) => count,
                None => {
                    response.content(format!("Looks like the word `{word}` has never been recorded in my database :("));
                    return ctx.response_send(response).await;
                }
            };

            table.push_record([&word, "", &total_times_said.separate_with_commas()]);
        }

        let description = format!("```\n{}\n```", table_style(table, self.style.as_ref()));

        // if let Some(word) = self.word {
        //     writeln!(total_words, "You wanted to see stats for the word `{word}`...")?;
        // }

        response.content(description);
        ctx.response_send(response).await

        // let mut response = ctx.acknowledge_interaction(false).await?;
        // let luro_user = ctx.get_specified_user_or_author(self.user.as_ref()).await?;
        // let mut description = String::new();

        // if self.global.unwrap_or_default() {
        //     let (total_words_said, total_unique_words) = ctx.database.driver.count_total_words().await?;
        //     writeln!(description,"Total words recorded in my database: `{total_words_said}`")?;
        //     writeln!(description,"Total unique words recorded in my database: `{total_unique_words}`")?;
        // }

        // let mut wordcount: usize = Default::default();
        // let mut averagesize: usize = Default::default();
        // let mut wordsize: BTreeMap<usize, usize> = Default::default();
        // let mut words: BTreeMap<String, usize> = Default::default();
        // let mut content = String::new();
        // // How many items we should get
        // let limit = match self.limit {
        //     Some(limit) => limit.try_into().context("Failed to convert i64 into usize")?,
        //     None => 10,
        // };

        // if global {
        //     let mut most_said_words: BTreeMap<Id<UserMarker>, usize> = Default::default();
        //     let mut user_ids = vec![];

        //     for (id, user_data) in data {
        //         user_ids.push(id);

        //         wordcount += user_data.wordcount;
        //         averagesize += user_data.averagesize;

        //         for (word, count) in user_data.words.clone().into_iter() {
        //             *words.entry(word).or_insert(0) += count;
        //             *most_said_words.entry(id).or_insert(0) += count;
        //         }

        //         for (size, count) in user_data.wordsize.clone().into_iter() {
        //             *wordsize.entry(size).or_insert(0) += count;
        //         }
        //     }

        //     writeln!(
        //         content,
        //         "Words counted from a total of **{}** users and I am showing stats for **{limit}** users!\n-----",
        //         user_ids.len()
        //     )?;

        //     let mut high_score_users = Vec::from_iter(most_said_words);
        //     high_score_users.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        //     high_score_users.truncate(limit);

        //     for (user_number, (user, count)) in high_score_users.into_iter().enumerate() {
        //         writeln!(content, "{user_number}. <@{user}> has said `{count}` words!")?;
        //     }
        //     if content.len() > 3800 {
        //         content.truncate(3800);
        //         content.push_str("...")
        //     }
        //     writeln!(content, "-----")?;
        // } else {
        //     wordcount = luro_user.wordcount;
        //     averagesize = luro_user.averagesize;
        //     wordsize = luro_user.wordsize.clone();
        //     words = luro_user.words.clone();
        // };

        // let averagesize = averagesize.checked_div(wordcount).unwrap_or(0);
        // writeln!(
        //     content,
        //     "Approximately **{}** words have been said with an average of **{}** letters per word.",
        //     wordcount, averagesize
        // )?;

        // // Handle if a user is just interested in a word
        // if let Some(word) = self.word {
        //     match words.get(&word) {
        //         // If we are getting a single word, then we want to get it from the BTreeMap that is sorted by key
        //         Some(word_count) => {
        //             writeln!(
        //                 content,
        //                 "-----\nSpecifically, the word `{word}` has been said about `{word_count}` times!"
        //             )?;
        //             return ctx
        //                 .respond(|r| {
        //                     r.embed(|e| {
        //                         e.description(content)
        //                             .colour(accent_colour)
        //                             .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()))
        //                     })
        //                     .response_type(response)
        //                 })
        //                 .await;
        //         }
        //         None => {
        //             content = format!("The word `{word}` has never been said, as far as I can see!");
        //             return ctx.respond(|r| r.content(content).response_type(response)).await;
        //         }
        //     }
        // };

        // // Word size field
        // let mut word_size = String::new();
        // let mut number_lengths = vec![];
        // let mut common_word_length = Vec::from_iter(wordsize);
        // common_word_length.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        // common_word_length.truncate(limit);
        // // First loop is for calculating total length
        // for (length, total) in &common_word_length {
        //     // Convert to base 10
        //     let total = match usize::try_from(total.checked_ilog10().unwrap_or(1)) {
        //         Ok(total) => total + 1,
        //         Err(_) => continue,
        //     };

        //     let length = match usize::try_from(length.checked_ilog10().unwrap_or(1)) {
        //         Ok(length) => length + 1,
        //         Err(_) => continue,
        //     };

        //     number_lengths.push((total, length))
        // }

        // let padding = padding_calculator(number_lengths.clone());
        // // Now loop through again, using our calculated padding
        // for (length, total) in &common_word_length {
        //     let total_padding = padding.0;
        //     let length_padding = padding.1;

        //     writeln!(
        //         word_size,
        //         "- `{total:^total_padding$}` words with `{length:^length_padding$}` characters"
        //     )?;
        // }

        // // Most used words field
        // let mut most_used = String::new();
        // let mut most_used_words = Vec::from_iter(words);
        // most_used_words.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        // most_used_words.truncate(limit);

        // let mut number_lengths = vec![];
        // // First loop is for calculating total length
        // for (word, count) in &most_used_words {
        //     // Convert to base 10
        //     let count = match usize::try_from(count.checked_ilog10().unwrap_or(1)) {
        //         Ok(total) => total,
        //         Err(_) => continue,
        //     };
        //     number_lengths.push((word.len(), count))
        // }

        // let padding = padding_calculator(number_lengths.clone());
        // for (word, count) in &most_used_words {
        //     let word_padding = padding.0;
        //     let count_padding = padding.1;

        //     writeln!(
        //         most_used,
        //         "- `{word:^word_padding$}` words with `{count:^count_padding$}` characters"
        //     )?;
        // }

        // ctx.respond(|r| {
        //     r.embed(|embed| {
        //         embed
        //             .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()))
        //             .description(content)
        //             .field(|field| field.field("Word Length", &word_size, true))
        //             .field(|field| field.field("Most used words", &most_used, true))
        //             .footer(|footer| footer.text(""))
        //             .colour(accent_colour)
        //     })
        //     .response_type(response)
        // })
        // .await
    }
}
