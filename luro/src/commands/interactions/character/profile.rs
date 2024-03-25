use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

use crate::models::interaction::{InteractionContext, InteractionResult};

use super::character_response;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "profile", desc = "Fetch a user's character profile")]
pub struct Command {
    /// The fursona to get
    pub name: String,
    /// The type of profile to fetch. Defaults to the channel type.
    nsfw: Option<bool>,
    /// Fetch the character name from someone else.
    user: Option<Id<UserMarker>>,
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        let user = ctx.author_or_user(self.user).await?;

        let nsfw = match self.nsfw {
            Some(nsfw) => match ctx.channel().nsfw {
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
            None => ctx.channel().nsfw.unwrap_or_default(),
        };

        let character = match ctx.database().fetch_character(user.user_id(), &self.name).await? {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for character in ctx.database().fetch_characters(user.user_id()).await? {
                    writeln!(characters, "- {}: {}", character.name, character.sfw_summary)?
                }

                let response = format!("I'm afraid that user <@{}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}",user.user_id, self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        let response = character_response(ctx.clone(), &character, &user, nsfw).await;
        ctx.response_send(response).await
    }
}
