use luro_model::database_driver::LuroDatabaseDriver;
use rand::seq::SliceRandom;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::component::ButtonStyle;

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "get", desc = "Get an image relating to a character")]
pub struct Get {
    #[command(desc = "The character to get", autocomplete = true)]
    pub name: String,
    /// The image ID to get
    id: Option<i64>,
    /// Is this a NSFW image?
    pub nsfw: Option<bool>,
    /// Do you just want to get favs?
    fav: Option<bool>,
}

impl LuroCommand for Get {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        let user_id = ctx.interaction.author_id().unwrap();

        let user_data = ctx.framework.database.get_user(&user_id).await?;
        if user_data.characters.is_empty() {
            return ctx
                .respond(|r| {
                    r.content(format!("Sorry, <@{user_id}> has no character profiles configured!"))
                        .ephemeral()
                })
                .await;
        }

        let character = match user_data.characters.get(&self.name) {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        let mut sfw_images = vec![];
        let mut nsfw_images = vec![];
        let mut sfw_favs = vec![];
        let mut nsfw_favs = vec![];

        for image in character.images.values() {
            match image.nsfw {
                true => {
                    if image.fav {
                        nsfw_favs.push(image)
                    }
                    nsfw_images.push(image)
                }
                false => {
                    if image.fav {
                        sfw_favs.push(image)
                    }
                    sfw_images.push(image)
                }
            }
        }

        let nsfw = self
            .nsfw
            .unwrap_or(ctx.interaction.channel.as_ref().unwrap().nsfw.unwrap_or_default());
        let img = if let Some(id) = self.id {
            character.images.get(&(id as usize))
        } else {
            let mut rng = rand::thread_rng();
            if self.fav.unwrap_or_default() {
                if nsfw {
                    if let Some(fav_img) = nsfw_favs.choose(&mut rng) {
                        Some(*fav_img)
                    } else {
                        sfw_favs.choose(&mut rng).copied()
                    }
                } else {
                    sfw_favs.choose(&mut rng).copied()
                }
            } else if nsfw {
                if let Some(img) = nsfw_images.choose(&mut rng) {
                    Some(*img)
                } else {
                    sfw_images.choose(&mut rng).copied()
                }
            } else {
                sfw_images.choose(&mut rng).copied()
            }
        };

        match img {
            Some(selected_img) => {
                if let Some((id, _)) = character.images.iter().find(|(_, img)| img == &selected_img) {
                    let footer = format!(
                        "Image ID: {id} | Total SFW Images: {} ({}F) | Total NSFW Images: {} ({}F)",
                        sfw_images.len(),
                        sfw_favs.len(),
                        nsfw_images.len(),
                        nsfw_favs.len()
                    );

                    embed.footer(|f| f.text(footer));
                }

                if let Some(source) = &selected_img.source {
                    embed.url(source);
                }

                embed.image(|img| img.url(selected_img.url.clone()));
                if !selected_img.name.is_empty() {
                    embed.title(selected_img.name.clone());
                }
            }
            None => return ctx.respond(|r| r.content("No images found!").ephemeral()).await,
        }

        embed.author(|author| {
            author
                .name(format!("Character by {}", user_data.name()))
                .icon_url(user_data.avatar())
        });

        ctx.respond(|r| {
            r.add_embed(embed).components(|components| {
                components.action_row(|row| {
                    if nsfw {
                        row.button(|button| {
                            button
                                .custom_id("character-image-nsfw")
                                .label("More NSFW!")
                                .style(ButtonStyle::Secondary)
                        });
                    }
                    row.button(|button| {
                        button
                            .custom_id("character-image-sfw")
                            .label("More SFW!")
                            .style(ButtonStyle::Secondary)
                    })
                })
            })
        })
        .await
    }
}
