use anyhow::Context;
use luro_model::{database_driver::LuroDatabaseDriver, user::character::CharacterImage};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "image", desc = "Set the primary image for this character")]
pub struct Image {
    #[command(desc = "The character to get", autocomplete = true)]
    name: String,
    /// The URL the image should be set to
    img: String,
    /// Is this a NSFW image?
    nsfw: bool,
    /// Do you want this image to show up as one of the main images in your profile?
    fav: bool,
    /// Overwrite an image? `0` is the primary image!
    overwrite: Option<i64>,
}

impl LuroCommand for Image {
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
            Some(character) => {
                let image = CharacterImage {
                    url: self.img,
                    nsfw: self.nsfw,
                    fav: self.fav,
                };
                let key = match self.overwrite {
                    Some(id) => id as usize,
                    None => character.images.len() + 1,
                };
                character.images.insert(key, image)
            }
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        ctx.framework.database.modify_user(&user_id, &user_data).await?;

        ctx.respond(|r| r.content("Done!").ephemeral()).await
    }
}
