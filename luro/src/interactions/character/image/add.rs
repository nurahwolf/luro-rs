use anyhow::Context;
use luro_model::{database_driver::LuroDatabaseDriver, user::character::CharacterImage};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Set the primary image for this character")]
pub struct Add {
    #[command(desc = "The character to get", autocomplete = true)]
    name: String,
    /// The name of the image
    img: String,
    /// The URL the image should be set to
    url: String,
    /// Is this a NSFW image?
    nsfw: bool,
    /// Do you want this image to show up as one of the main images in your profile?
    fav: bool,
    /// Overwrite an image by ID
    overwrite: Option<i64>,
    /// The source URL for the image
    source: Option<String>,
}

impl LuroCommand for Add {
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

        let (image, key) = match user_data.characters.get_mut(&self.name) {
            Some(character) => {
                let image = CharacterImage {
                    url: self.url,
                    nsfw: self.nsfw,
                    fav: self.fav,
                    name: self.img,
                    source: self.source,
                };
                let key = match self.overwrite {
                    Some(id) => id as usize,
                    None => character.images.len() + 1,
                };
                character.images.insert(key, image.clone());
                (image, key)
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
        let mut embed = ctx.default_embed().await;
        embed.footer(|f| f.text(format!("Image ID: {key}")));

        if let Some(source) = &image.source {
            embed.url(source);
        }

        embed.image(|img| img.url(image.url.clone()));
        if !image.name.is_empty() {
            embed.title(image.name.clone());
        }

        embed.author(|author| {
            author
                .name(format!("Character by {}", user_data.name()))
                .icon_url(user_data.avatar())
        });

        ctx.respond(|r| r.add_embed(embed).ephemeral()).await
    }
}
