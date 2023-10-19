use luro_framework::{CommandInteraction, LuroCommand};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};


use super::character_response;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "profile", desc = "Fetch a user's character profile")]
pub struct Profile {
    /// The fursona to get
    pub name: String,
    /// The type of profile to fetch. Defaults to the channel type.
    nsfw: Option<bool>,
    /// Fetch the character name from someone else.
    user: Option<ResolvedUser>,
}

impl LuroCommand for Profile {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let user = ctx.get_specified_user_or_author(self.user.as_ref()).await?;

        let nsfw = match self.nsfw {
            Some(nsfw) => match ctx.channel.nsfw {
                Some(channel_nsfw) => match !channel_nsfw && nsfw {
                    true => {
                        return ctx
                            .respond(|r| r.content("You can't get a NSFW profile in a SFW channel, dork!"))
                            .await
                    }
                    false => nsfw,
                },
                None => nsfw,
            },
            None => ctx.channel.nsfw.unwrap_or_default(),
        };

        let character = match user.fetch_character(&self.name).await? {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for (character_name, character) in user.fetch_characters().await? {
                    writeln!(characters, "- {character_name}: {}", character.sfw_summary)?
                }

                let response = format!("I'm afraid that user <@{}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}",user.user_id, self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        character_response(ctx, &character, &user, nsfw).await
    }
}
