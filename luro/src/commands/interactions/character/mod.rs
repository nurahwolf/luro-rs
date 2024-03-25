use luro_model::{
    builders::{ComponentBuilder, InteractionResponseBuilder},
    character::CharacterProfile,
};
use twilight_interactions::command::{AutocompleteValue, CommandModel, CreateCommand};
use twilight_model::{
    application::command::{CommandOptionChoice, CommandOptionChoiceValue},
    channel::message::component::ButtonStyle,
    http::interaction::InteractionResponseType,
};

use crate::models::interaction::{InteractionContext, InteractionError, InteractionResult};

// mod components;
// mod create;
// mod fetish;
// mod icon;
// mod image;
mod profile;
// mod proxy;
// mod send;

#[derive(CommandModel, CreateCommand)]
#[command(name = "character", desc = "Show off your character!")]
pub enum Command {
    #[command(name = "profile")]
    Profile(profile::Command),
    // #[command(name = "create")]
    // Create(create::Command),
    // #[command(name = "fetish")]
    // Fetish(fetish::Command),
    // #[command(name = "proxy")]
    // Proxy(proxy::Command),
    // #[command(name = "icon")]
    // Icon(icon::Command),
    // #[command(name = "img")]
    // Img(image::Command),
    // #[command(name = "send")]
    // Send(send::Command),
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        match self {
            // Self::Create(cmd) => cmd.handle_command(ctx).await,
            // Self::Fetish(cmd) => cmd.handle_command(ctx).await,
            // Self::Icon(cmd) => cmd.handle_command(ctx).await,
            // Self::Img(cmd) => cmd.handle_command(ctx).await,
            Self::Profile(cmd) => cmd.handle_command(ctx).await,
            // Self::Proxy(cmd) => cmd.handle_command(ctx).await,
            // Self::Send(cmd) => cmd.handle_command(ctx).await,
        }
    }

    async fn handle_modal(ctx: &mut InteractionContext) -> InteractionResult<()> {
        let nsfw = ctx.interaction.channel.map(|x| x.nsfw).flatten().unwrap_or_default();

        // Modal
        let character_name = ctx.parse_field_required("character-name")?;
        let sfw_icon = ctx.parse_field_required("character-icon")?;
        let sfw_summary = ctx.parse_field_required("character-sfw-summary")?;
        let nsfw_summary = ctx.parse_field("character-nsfw-summary")?;
        let sfw_description = ctx.parse_field_required("character-sfw-description")?;
        let nsfw_description = ctx.parse_field("character-nsfw-description")?;

        let character = ctx.database().fetch_character(ctx.author_id(), character_name).await?;
        let character = match character {
            Some(mut character) => {
                character.sfw_description = sfw_description.to_owned();
                character.sfw_summary = sfw_summary.to_owned();

                if let Some(nsfw_description) = nsfw_description {
                    character.nsfw_description = Some(nsfw_description.to_owned());
                }

                if let Some(nsfw_summary) = nsfw_summary {
                    character.nsfw_summary = Some(nsfw_summary.to_owned());
                }

                character
            }
            None => CharacterProfile {
                user_id: ctx.author_id(),
                colour: None,
                nickname: None,
                name: character_name.to_owned(),
                nsfw_description: nsfw_description.map(|x| x.to_owned()),
                nsfw_icon: Default::default(),
                nsfw_summary: nsfw_summary.map(|x| x.to_owned()),
                prefix: Default::default(),
                sfw_description: sfw_description.to_owned(),
                sfw_icon: sfw_icon.to_owned(),
                sfw_summary: sfw_summary.to_owned(),
            },
        };

        ctx.database().update_character(&character).await?;
        ctx.response_send(&character_response(ctx, &character, nsfw).await).await
    }

    async fn handle_component(ctx: &mut InteractionContext) -> InteractionResult<()> {
        match ctx.command_name() {
            // "character-menu-open" => character_menu(ctx, true).await,
            // "character-menu-close" => character_menu(ctx, false).await,
            // "character-description" => self.character_description_button(ctx).await,
            // "character-edit" => self.character_edit_button(ctx).await,
            // "character-image" => self.character_cycle_image_button(ctx).await,
            // "character-image-nsfw" => self.character_image_button(ctx, true).await,
            // "character-image-sfw" => self.character_image_button(ctx, false).await,
            // "character-fetish" => self.character_fetish_button(ctx).await,
            name => Err(InteractionError::NotComponent),
        }
    }

    async fn handle_autocomplete(ctx: &mut InteractionContext) -> InteractionResult<()> {
        let characters = ctx.database().fetch_characters(ctx.author_id()).await?;

        let choices = match CharacterNameAutocomplete::from_interaction((ctx.interaction).into())?.name {
            AutocompleteValue::None => characters
                .into_iter()
                .map(|character| CommandOptionChoice {
                    name: character.name.clone(),
                    name_localizations: None,
                    value: CommandOptionChoiceValue::String(character.name),
                })
                .collect(),
            AutocompleteValue::Focused(input) => characters
                .into_iter()
                .filter_map(|character| match character.name.contains(&input) {
                    true => Some(CommandOptionChoice {
                        name: character.name.clone(),
                        name_localizations: None,
                        value: CommandOptionChoiceValue::String(character.name),
                    }),
                    false => None,
                })
                .collect(),
            AutocompleteValue::Completed(_) => vec![],
        };

        ctx.respond(|response| {
            response
                .choices(choices.into_iter())
                .response_type(InteractionResponseType::ApplicationCommandAutocompleteResult)
        })
        .await
    }
}

#[derive(CommandModel)]
#[command(autocomplete = true)]
pub struct CharacterNameAutocomplete {
    name: AutocompleteValue<String>,
}

pub async fn character_response(ctx: &InteractionContext, character: &CharacterProfile, nsfw: bool) -> InteractionResponseBuilder {
    let mut response = InteractionResponseBuilder::default();
    let character_images = ctx
        .database()
        .fetch_character_images(&character.name, ctx.author_id())
        .await
        .unwrap_or_default();
    let character_icon = match nsfw {
        true => character.nsfw_icon.as_ref().unwrap_or(&character.sfw_icon),
        false => &character.sfw_icon,
    };

    let mut nsfw_images = vec![];
    let mut sfw_images = vec![];
    let mut nsfw_favs = vec![];
    let mut sfw_favs = vec![];

    for image in character_images {
        tracing::debug!("Image: {image:#?}");
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

    let character_image = match nsfw {
        true => match !nsfw_favs.is_empty() {
            true => nsfw_favs.choose(&mut thread_rng()),
            false => nsfw_images.choose(&mut thread_rng()),
        },
        false => match !sfw_favs.is_empty() {
            true => sfw_favs.choose(&mut thread_rng()),
            false => sfw_images.choose(&mut thread_rng()),
        },
    };

    response
        .embed(|embed| {
            match nsfw {
                true => match &character.nsfw_summary {
                    Some(nsfw_summary) => {
                        embed
                            .create_field("NSFW Summary", nsfw_summary, true)
                            .create_field("SFW Summary", &character.sfw_summary, true)
                    }
                    None => embed.create_field("SFW Summary", &character.sfw_summary, true),
                },
                false => embed.create_field("SFW Summary", &character.sfw_summary, true),
            };
            if let Some(prefix) = &character.prefix {
                embed.create_field("Prefix", &format!("`{prefix}`"), false);
            }
            if let Some(character_image) = character_image {
                embed.image(|i| i.url(character_image.url.clone()));
            }
            embed
                .author(|a| {
                    a.icon_url(user.avatar_url())
                        .name(format!("{} - [{}]", character.name, user.name()))
                })
                .colour(character.colour.unwrap_or(ctx.accent_colour()))
                .thumbnail(|t| t.url(character_icon))
        })
        .components(|c| {
            *c = components(false, nsfw);
            c
        });
    response
}

/// Generate a collection of components, depending on the passed settings
///
/// edit_menu: True shows the menu, false shows the closed menu
/// nsfw: Show fetish buttons
fn components(edit_menu: bool, nsfw: bool) -> ComponentBuilder {
    let mut components = ComponentBuilder::default();
    // Row 1
    components.action_row(|row| {
        match nsfw {
            true => row
                .button(|button| button.custom_id("character-fetish").label("Fetishes").style(ButtonStyle::Secondary))
                .button(|button| {
                    button
                        .custom_id("character-image-nsfw")
                        .label("Cycle Image")
                        .style(ButtonStyle::Secondary)
                }),
            false => row.button(|button| {
                button
                    .custom_id("character-image")
                    .label("Cycle Image")
                    .style(ButtonStyle::Secondary)
            }),
        };
        row.button(|button| {
            button
                .custom_id("character-description")
                .label("Show Detailed Description")
                .style(ButtonStyle::Secondary)
        })
        .button(|button| {
            button
                .custom_id("character-delete")
                .label("Delete Character")
                .style(ButtonStyle::Danger)
        });
        match edit_menu {
            true => row.button(|button| {
                button
                    .custom_id("character-menu-close")
                    .label("Close Menu")
                    .style(ButtonStyle::Primary)
            }),
            false => row.button(|button| {
                button
                    .custom_id("character-menu-open")
                    .label("Edit Menu")
                    .style(ButtonStyle::Primary)
            }),
        }
    });
    // Row 2
    if edit_menu {
        components.action_row(|row| {
            row.button(|button| {
                button
                    .custom_id("character-add-prefix")
                    .label("Add Prefix")
                    .style(ButtonStyle::Secondary)
            })
            .button(|button| {
                button
                    .custom_id("character-add-image")
                    .label("Add Images")
                    .style(ButtonStyle::Secondary)
            })
            .button(|button| {
                button
                    .custom_id("character-add-icon")
                    .label("Add Icons")
                    .style(ButtonStyle::Secondary)
            });
            if nsfw {
                row.button(|button| {
                    button
                        .custom_id("character-add-fetish")
                        .label("Add Fetishes")
                        .style(ButtonStyle::Secondary)
                });
            }
            row.button(|button| {
                button
                    .custom_id("character-edit")
                    .label("Update Description/Summary")
                    .style(ButtonStyle::Secondary)
            })
        });
    }

    components
}

impl Command {
    pub fn character_name(&self) -> &str {
        match self {
            // Self::Img(img) => match img {
            //     // crate::commands::character::image::Image::Add(cmd) => cmd.character,
            //     crate::commands::character::image::Image::Get(cmd) => &cmd.character,
            // },
            Self::Profile(cmd) => &cmd.name,
            // Self::Create(cmd) => &cmd.name,
            // Self::Proxy(cmd) => &cmd.name,
            // Self::Icon(cmd) => &cmd.name,
            // Self::Send(cmd) => &cmd.name,
        }
    }
}
