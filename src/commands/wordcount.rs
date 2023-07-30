use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_util::builder::embed::EmbedAuthorBuilder;

use super::LuroCommand;
use crate::{models::UserData, responses::LuroSlash};
use std::fmt::Write;

#[derive(CommandModel, CreateCommand)]
#[command(name = "wordcount", desc = "Get some stats on the bullshit someone has posted.")]
pub struct WordcountCommand {
    /// The user to get the stats of
    user: ResolvedUser,
    /// A particular word to search word
    word: Option<String>
}

#[async_trait]
impl LuroCommand for WordcountCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut content = "**Word Totals**\n".to_owned();
        let user_data = UserData::get_user_settings(&ctx.luro, &self.user.resolved.id).await?;
        let embed = ctx.default_embed().await?;

        let user_name = match self.user.member {
            Some(ref member) => member.clone().nick.unwrap_or(self.user.resolved.name.clone()),
            None => self.user.resolved.name.clone()
        };
        let user_avatar = self.get_interaction_member_avatar(self.user.member.clone(), &ctx.interaction.guild_id, &self.user.resolved);
        let author = EmbedAuthorBuilder::new(user_name).url(user_avatar);


        if let Some(word) = self.word {
            match user_data.wordcount.get(&word) {
                Some(word_count) => {
                    content = format!("**{word}:** `{word_count}`")
                },
                None => return ctx.content(format!("Sorry! That user does not have the word {word} saved! :(")).respond().await
            }
            
        } else {
            for (word, amount) in user_data.wordcount.iter() {
                writeln!(content, "`{word}`: `{amount}`")?
            }
        }

        ctx.embed(embed.description(content).author(author).build())?.respond().await
    }
}
