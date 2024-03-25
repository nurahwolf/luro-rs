use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "icon", desc = "Set the primary icon for this character")]
pub struct Command {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    pub name: String,
    /// The URL the icon should be set to
    icon: String,
    /// The URL a NSFW icon
    nsfw_icon: Option<String>,
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        let mut character = match ctx.database().fetch_character(ctx.author_id(), &self.name).await? {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for character in ctx.database().fetch_characters(ctx.author_id()).await? {
                    writeln!(characters, "- {}: {}", character.name, character.sfw_summary)?
                }

                let response = format!("I'm afraid that user <@{}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}",ctx.author.user_id, self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        character.sfw_icon = self.icon;
        character.nsfw_icon = self.nsfw_icon;
        ctx.database().update_character(&character, ctx.author_id()).await?;

        ctx.respond(|r| r.content("Updated!").ephemeral()).await
    }
}
