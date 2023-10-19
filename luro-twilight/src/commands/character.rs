use std::fmt::Write;
use anyhow::Context;
use luro_database::{DatabaseInteraction, LuroCharacter, LuroUser, LuroCharacterFetishCategory};
use luro_framework::{
    CommandInteraction, ComponentInteraction, InteractionTrait, Luro, ModalInteraction, {CreateLuroCommand, LuroCommand},
};
use twilight_interactions::command::{AutocompleteValue, CommandModel, CreateCommand};
use twilight_model::{
    application::command::{CommandOptionChoice, CommandOptionChoiceValue},
    http::interaction::InteractionResponseType, channel::message::component::ButtonStyle,
};

use self::{create::Create, icon::Icon, profile::Profile, proxy::Proxy, send::CharacterSend};

mod create;
// mod fetish;
mod icon;
mod profile;
mod proxy;
pub mod send;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "character", desc = "Show off your character!")]
pub enum Character {
    #[command(name = "profile")]
    Profile(Profile),
    #[command(name = "create")]
    Create(Create),
    // #[command(name = "fetish")]
    // Fetish(Fetish),
    #[command(name = "proxy")]
    Proxy(Proxy),
    #[command(name = "icon")]
    Icon(Icon),
    #[command(name = "send")]
    Send(CharacterSend),
}

impl CreateLuroCommand for Character {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        match self {
            Self::Profile(command) => command.interaction_command(ctx).await,
            Self::Create(command) => command.interaction_command(ctx).await,
            // Self::Fetish(command) => command.interaction_command(ctx).await,
            Self::Proxy(command) => command.interaction_command(ctx).await,
            Self::Icon(command) => command.interaction_command(ctx).await,
            Self::Send(command) => command.interaction_command(ctx).await,
        }
    }

    async fn interaction_modal(ctx: ModalInteraction) -> anyhow::Result<()> {
        let user_id = ctx.author_id();
        let nsfw: bool = ctx.channel.nsfw.unwrap_or_default();
        let user = ctx.fetch_user(&user_id).await?;

        // Modal
        let character_name = ctx.parse_field_required("character-name")?;
        let sfw_summary = ctx.parse_field_required("character-sfw-summary")?;
        let nsfw_summary = ctx.parse_field("character-nsfw-summary")?;
        let sfw_description = ctx.parse_field_required("character-sfw-description")?;
        let nsfw_description = ctx.parse_field("character-nsfw-description")?;

        let character = user.fetch_character(character_name).await?;
        let mut character = match character {
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
            None => LuroCharacter {
                name: character_name.to_owned(),
                nsfw_description: nsfw_description.map(|x| x.to_owned()),
                nsfw_icons: Default::default(),
                nsfw_summary: nsfw_summary.map(|x| x.to_owned()),
                prefix: Default::default(),
                sfw_description: sfw_description.to_owned(),
                sfw_icons: Default::default(),
                sfw_summary: sfw_summary.to_owned(),
                user_id: ctx.author_id().get() as i64,
                db: ctx.database.clone(),
            },
        };

        character = user.update_character_text(character).await?;
        character_response(ctx, &character, &user, nsfw).await
    }

    async fn interaction_component(self, ctx: ComponentInteraction, _invoking_interaction: DatabaseInteraction) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        let user = ctx.fetch_user(&ctx.author_id()).await?;
        let character_name = match self {
            Character::Profile(data) => data.name,
            Character::Create(data) => data.name,
            _ => return ctx.respond(|r| r.content("Invalid command").ephemeral()).await,
        };
        let character = user
            .fetch_character(&character_name)
            .await?
            .context("Could not find that character! Was it deleted?")?;

        embed.title(format!("{character_name}'s Fetishes"));

        let mut fav = String::new();
        let mut love = String::new();
        let mut like = String::new();
        let mut neutral = String::new();
        let mut dislike = String::new();
        let mut hate = String::new();
        let mut limits = String::new();

        for fetish in &character.get_fetishes().await? {
            match fetish.category {
                LuroCharacterFetishCategory::Favourite => writeln!(fav, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Love => writeln!(love, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Like => writeln!(like, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Neutral => writeln!(neutral, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Dislike => writeln!(dislike, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Hate => writeln!(hate, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Limit => writeln!(limits, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
            }
        }

        if !fav.is_empty() {
            embed.create_field("Favourites", &fav, false);
        }

        if !love.is_empty() {
            embed.create_field("Love", &love, false);
        }

        if !like.is_empty() {
            embed.create_field("Like", &like, false);
        }

        if !neutral.is_empty() {
            embed.create_field("Neutral", &neutral, false);
        }

        if !dislike.is_empty() {
            embed.create_field("Dislike", &dislike, false);
        }

        if !hate.is_empty() {
            embed.create_field("Hate", &hate, false);
        }

        if !limits.is_empty() {
            embed.create_field("Limits", &limits, false);
        }

        ctx.respond(|r| r.add_embed(embed).ephemeral()).await
    }

    async fn interaction_autocomplete(ctx: CommandInteraction) -> anyhow::Result<()> {
        let user = ctx.fetch_user(&ctx.author_id()).await?;
        let characters = user.fetch_characters().await?;

        let choices = match CharacterNameAutocomplete::from_interaction((*ctx.data.clone()).into())?.name {
            AutocompleteValue::None => characters
                .keys()
                .map(|name| CommandOptionChoice {
                    name: name.clone(),
                    name_localizations: None,
                    value: CommandOptionChoiceValue::String(name.clone()),
                })
                .collect(),
            AutocompleteValue::Focused(input) => characters
                .keys()
                .filter_map(|name| match name.contains(&input) {
                    true => Some(CommandOptionChoice {
                        name: name.clone(),
                        name_localizations: None,
                        value: CommandOptionChoiceValue::String(name.clone()),
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

pub async fn character_response<T: Luro>(ctx: T, character: &LuroCharacter, user: &LuroUser, nsfw: bool) -> anyhow::Result<()> {
    let accent_colour = ctx.accent_colour().await;
    ctx.respond(|response| {
        response.embed(|embed| {
            embed
                .title(format!("{}'s Profile", character.name))
                .author(|a| a.icon_url(user.avatar()).name(format!("Character by {}", user.name())))
                .colour(accent_colour);

            if nsfw {
                match &character.nsfw_summary {
                    Some(summary) => embed.create_field("NSFW Summary", summary, false),
                    None => embed.create_field("Summary", &character.sfw_summary, false),
                };

                match &character.nsfw_description {
                    Some(description) => embed.description(description),
                    None => embed.description(character.sfw_description.clone()),
                };
            } else {
                embed
                    .description(character.sfw_description.clone())
                    .create_field("Summary", &character.sfw_summary, false);
            }

            if let Some(prefix) = &character.prefix {
                embed.create_field(
                    "Character Prefixes",
                    prefix,
                    false,
                );
            }

            embed
        }).components(|components| {
            components.action_row(|row| {
                row.button(|button| {
                    button
                        .custom_id("character-image")
                        .label("Cycle Image")
                        .style(ButtonStyle::Secondary)
                })
                .button(|button| {
                    button
                        .custom_id("character-update")
                        .label("Update Character")
                        .style(ButtonStyle::Secondary)
                });
                // if nsfw && !character.fetishes.is_empty() {
                //     response.components(|components| {
                //         components.action_row(|row| {
                //             row.button(|button| button.custom_id("character-fetish").label("Fetishes").style(ButtonStyle::Danger))
                //         })
                //     });
                // }
                row
            })
        })
    })
    .await
}
