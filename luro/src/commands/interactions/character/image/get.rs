use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::component::ButtonStyle;

use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(CommandModel, CreateCommand)]
#[command(name = "get", desc = "Get an image relating to a character")]
pub struct Command {
    #[command(desc = "The character to get", autocomplete = true)]
    pub character: String,
    /// The image ID to get
    id: Option<i64>,
    /// Is this a NSFW image?
    nsfw: Option<bool>,
    /// Do you just want to get favs?
    fav: Option<bool>,
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        let nsfw = self.nsfw.unwrap_or(ctx.channel().nsfw.unwrap_or_default());
        let character_images = ctx.database().fetch_character_images(&self.character, ctx.author_id()).await?;

        let mut nsfw_images = vec![];
        let mut sfw_images = vec![];
        let mut nsfw_favs = vec![];
        let mut sfw_favs = vec![];

        for image in character_images {
            match image.nsfw {
                true => match image.favourite {
                    true => nsfw_favs.push(image),
                    false => nsfw_images.push(image),
                },
                false => match image.favourite {
                    true => sfw_favs.push(image),
                    false => sfw_images.push(image),
                },
            }
        }

        let selected_image = match self.id {
            Some(image_id) => {
                ctx.database()
                    .fetch_character_image(&self.character, ctx.author_id(), image_id)
                    .await?
            }
            None => match nsfw {
                true => match self.fav.unwrap_or_default() {
                    true => nsfw_favs.choose(&mut thread_rng()).cloned(),
                    false => nsfw_images.choose(&mut thread_rng()).cloned(),
                },
                false => match self.fav.unwrap_or_default() {
                    true => sfw_favs.choose(&mut thread_rng()).cloned(),
                    false => sfw_images.choose(&mut thread_rng()).cloned(),
                },
            },
        };

        let selected_image = match selected_image {
            Some(img) => img,
            None => {
                return ctx
                    .respond(|r| {
                        r.content(format!(
                            "Sorry, <@{}> has no images configured for this character!",
                            ctx.author.user_id
                        ))
                        .ephemeral()
                    })
                    .await
            }
        };

        ctx.respond(|response| {
            response
                .embed(|embed| {
                    embed
                        .colour(ctx.accent_colour())
                        .footer(|f| {
                            f.text(format!(
                                "Image ID: {} | Total SFW Images: {} ({}F) | Total NSFW Images: {} ({}F)",
                                selected_image.img_id,
                                sfw_images.len(),
                                sfw_favs.len(),
                                nsfw_images.len(),
                                nsfw_favs.len()
                            ))
                        })
                        .title(selected_image.name)
                        .image(|img| img.url(selected_image.url))
                        .author(|author| {
                            author
                                .name(format!("Profile by {}", ctx.author.name()))
                                .icon_url(ctx.author.avatar_url())
                        });

                    if let Some(source) = &selected_image.source {
                        embed.url(source);
                    }

                    embed
                })
                .components(|components| {
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
