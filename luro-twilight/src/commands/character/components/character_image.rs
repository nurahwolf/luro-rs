use anyhow::Context;
use luro_framework::{ComponentInteraction, Luro};
use luro_model::types::CommandResponse;
use rand::{seq::SliceRandom, thread_rng};
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType};

impl crate::commands::character::Character {
    pub async fn character_image_button(
        &self,
        ctx: ComponentInteraction,
        invoking_interaction: Interaction,
        nsfw: bool,
    ) -> anyhow::Result<CommandResponse> {
        let original_author_id = invoking_interaction
            .author_id()
            .context("Expected to get user ID from interaction")?;
        let original_author = ctx.fetch_user(original_author_id).await?;
        let character_name = self.character_name();
        let character_images = ctx
            .database
            .driver
            .character_fetch_images(character_name, original_author_id)
            .await?;
        let character = match ctx.database.user_fetch_character(original_author_id, character_name).await? {
            Some(character) => character,
            None => return ctx.respond(|r|r.content(format!("Sorry, could not find the character {character_name} in my database. The user might have deleted this profile, sorry!")).ephemeral()).await,
        };

        let mut nsfw_images = vec![];
        let mut sfw_images = vec![];

        for image in character_images {
            match image.nsfw {
                true => nsfw_images.push(image),
                false => sfw_images.push(image),
            }
        }

        let character_image = match nsfw {
            true => nsfw_images.choose(&mut thread_rng()),
            false => sfw_images.choose(&mut thread_rng()),
        };

        let character_image = match character_image {
            Some(img) => img,
            None => {
                return ctx
                    .respond(|r| r.content("Sorry, that character has no more images configured").ephemeral())
                    .await
            }
        };

        let description = match nsfw {
            true => format!(
                "<:plus:1175893813261250570> Total NSFW Images: `{}` | Total SFW Images: `{}`\n<:plus:1175893813261250570> Character by <@{original_author_id}>",
                nsfw_images.len(),
                sfw_images.len()
            ),
            false => format!("<:plus:1175893813261250570> Total SFW Images: `{}`\n<:plus:1175893813261250570> Character by <@{original_author_id}>", sfw_images.len()),
        };
        ctx.respond(|r| {
            r.embed(|e| {
                if let Some(source) = &character_image.source {
                    e.url(source);
                }
                e.colour(character.colour.unwrap_or(ctx.accent_colour()))
                    .author(|a| a.name(character.name).icon_url(original_author.avatar_url()))
                    .title(character_image.name.clone())
                    .description(description)
                    .image(|i| i.url(character_image.url.clone()))
                    .thumbnail(|t| {
                        t.url(match nsfw {
                            true => character.nsfw_icon.as_ref().unwrap_or(&character.sfw_icon),
                            false => &character.sfw_icon,
                        })
                    })
                    .footer(|f| {
                        f.text(match nsfw {
                            true => match character_image.favourite {
                                true => format!("[FAV | NSFW | Image ID: {}]", character_image.img_id,),
                                false => format!("[NSFW | Image ID: {}]", character_image.img_id,),
                            },
                            false => match character_image.favourite {
                                true => format!("[FAV | Image ID: {}]", character_image.img_id),
                                false => format!("[Image ID: {}]", character_image.img_id),
                            },
                        })
                    })
            })
            .response_type(InteractionResponseType::UpdateMessage)
        })
        .await
    }
}
