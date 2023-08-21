use anyhow::Context;
use luro_model::database::drivers::LuroDatabaseDriver;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "icon", desc = "Set the primary icon for this character")]
pub struct Icon {
    /// The fursona that should be proxied
    pub name: String,
    /// The URL the icon should be set to
    pub url: String
}

impl LuroCommand for Icon {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let user_id = ctx
            .interaction
            .author_id()
            .context("Expected to find the user running this command")?;

        let mut user_data = ctx.framework.database.get_user(&user_id).await?;
        if user_data.characters.is_empty() {
            return ctx
                .respond(|r| {
                    r.content(format!("Sorry, <@{user_id}> has no character profiles configured!"))
                        .ephemeral()
                })
                .await;
        }

        match user_data.characters.get_mut(&self.name) {
            Some(character) => character.icon = self.url,
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        ctx.framework.database.save_user(&user_id, &user_data).await?;

        ctx.respond(|r| r.content("Done!").ephemeral()).await
    }
}
