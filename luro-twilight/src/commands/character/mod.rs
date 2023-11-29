use luro_framework::{
    CommandInteraction, ComponentInteraction, Luro, ModalInteraction, {CreateLuroCommand, LuroCommand},
};
use luro_model::{
    builders::ComponentBuilder,
    response::{InteractionResponse, SimpleResponse},
    types::{CharacterProfile, CommandResponse, User},
};
use rand::{seq::SliceRandom, thread_rng};
use twilight_interactions::command::{AutocompleteValue, CommandModel, CreateCommand};
use twilight_model::{
    application::{
        command::{CommandOptionChoice, CommandOptionChoiceValue},
        interaction::Interaction,
    },
    channel::message::component::ButtonStyle,
    http::interaction::InteractionResponseType,
};

use self::components::character_menu;

mod create;
// mod fetish;
mod components;
mod icon;
mod image;
mod profile;
mod proxy;
mod send;

#[derive(CommandModel, CreateCommand)]
#[command(name = "character", desc = "Show off your character!")]
pub enum Character {
    #[command(name = "profile")]
    Profile(profile::Profile),
    #[command(name = "create")]
    Create(create::Create),
    // #[command(name = "fetish")]
    // Fetish(fetish::Fetish),
    #[command(name = "proxy")]
    Proxy(proxy::Proxy),
    #[command(name = "icon")]
    Icon(icon::Icon),
    #[command(name = "img")]
    Img(image::Image),
    #[command(name = "send")]
    Send(send::CharacterSend),
}

impl CreateLuroCommand for Character {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<CommandResponse> {
        match self {
            Self::Create(command) => command.interaction_command(ctx).await,
            // Self::Fetish(command) => command.interaction_command(ctx).await,
            Self::Icon(command) => command.interaction_command(ctx).await,
            Self::Img(command) => command.interaction_command(ctx).await,
            Self::Profile(command) => command.interaction_command(ctx).await,
            Self::Proxy(command) => command.interaction_command(ctx).await,
            Self::Send(command) => command.interaction_command(ctx).await,
        }
    }

    async fn interaction_modal(ctx: ModalInteraction) -> anyhow::Result<CommandResponse> {
        let nsfw: bool = ctx.channel.nsfw.unwrap_or_default();

        // Modal
        let character_name = ctx.parse_field_required("character-name")?;
        let sfw_icon = ctx.parse_field_required("character-icon")?;
        let sfw_summary = ctx.parse_field_required("character-sfw-summary")?;
        let nsfw_summary = ctx.parse_field("character-nsfw-summary")?;
        let sfw_description = ctx.parse_field_required("character-sfw-description")?;
        let nsfw_description = ctx.parse_field("character-nsfw-description")?;

        let character = ctx.database.user_fetch_character(ctx.author.user_id, character_name).await?;
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

        ctx.database.driver.character_update(&character, ctx.author.user_id).await?;
        let response = character_response(ctx.clone(), &character, &ctx.author, nsfw).await;
        ctx.response_send(response).await
    }

    async fn interaction_component(
        self,
        ctx: ComponentInteraction,
        invoking_interaction: Interaction,
    ) -> anyhow::Result<CommandResponse> {
        match ctx.command_name() {
            "character-menu-open" => character_menu(ctx, invoking_interaction, true).await,
            "character-menu-close" => character_menu(ctx, invoking_interaction, false).await,
            "character-description" => self.character_description_button(ctx, invoking_interaction).await,
            "character-edit" => self.character_edit_button(ctx, invoking_interaction).await,
            "character-image" => self.character_cycle_image_button(ctx, invoking_interaction).await,
            "character-image-nsfw" => self.character_image_button(ctx, invoking_interaction, true).await,
            "character-image-sfw" => self.character_image_button(ctx, invoking_interaction, false).await,
            "character-fetish" => self.character_fetish_button(ctx, invoking_interaction).await,
            name => ctx.simple_response(SimpleResponse::UnknownCommand(name)).await,
        }
    }

    async fn interaction_autocomplete(ctx: CommandInteraction) -> anyhow::Result<CommandResponse> {
        let characters = ctx.database.user_fetch_characters(ctx.author.user_id).await?;

        let choices = match CharacterNameAutocomplete::from_interaction((*ctx.data.clone()).into())?.name {
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

pub async fn character_response<T: Luro + Sync>(ctx: T, character: &CharacterProfile, user: &User, nsfw: bool) -> InteractionResponse {
    let mut response = InteractionResponse::default();
    let character_images = ctx
        .database()
        .driver
        .character_fetch_images(&character.name, user.user_id)
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

impl Character {
    pub fn character_name(&self) -> &str {
        match self {
            crate::commands::character::Character::Img(img) => match img {
                // crate::commands::character::image::Image::Add(cmd) => cmd.character,
                crate::commands::character::image::Image::Get(cmd) => &cmd.character,
            },
            crate::commands::character::Character::Profile(cmd) => &cmd.name,
            crate::commands::character::Character::Create(cmd) => &cmd.name,
            crate::commands::character::Character::Proxy(cmd) => &cmd.name,
            crate::commands::character::Character::Icon(cmd) => &cmd.name,
            crate::commands::character::Character::Send(cmd) => &cmd.name,
        }
    }
}
