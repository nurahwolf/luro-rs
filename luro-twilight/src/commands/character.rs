use anyhow::Context;
use luro_framework::{
    command::{CreateLuroCommand, ExecuteLuroCommand},
    interactions::InteractionTrait,
    CommandInteraction, ComponentInteraction, Luro, ModalInteraction,
};
use luro_model::user::character::{CharacterProfile, FetishCategory};
use twilight_model::{application::command::{CommandOptionChoice, CommandOptionChoiceValue}, http::interaction::InteractionResponseType};
use std::{collections::btree_map::Entry, fmt::Write};
use twilight_interactions::command::{CommandModel, CreateCommand, AutocompleteValue};

use self::{create::Create, fetish::Fetish, icon::Icon, profile::Profile, proxy::Proxy, send::CharacterSend};

mod create;
mod fetish;
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
    #[command(name = "fetish")]
    Fetish(Fetish),
    #[command(name = "proxy")]
    Proxy(Proxy),
    #[command(name = "icon")]
    Icon(Icon),
    #[command(name = "send")]
    Send(CharacterSend),
}

impl CreateLuroCommand for Character {}

impl ExecuteLuroCommand for Character {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        match self {
            Self::Profile(command) => command.interaction_command(ctx).await,
            Self::Create(command) => command.interaction_command(ctx).await,
            Self::Fetish(command) => command.interaction_command(ctx).await,
            Self::Proxy(command) => command.interaction_command(ctx).await,
            Self::Icon(command) => command.interaction_command(ctx).await,
            Self::Send(command) => command.interaction_command(ctx).await,
        }
    }

    async fn interaction_modal(ctx: ModalInteraction) -> anyhow::Result<()> {
        let user_id = ctx.author_id();
        let nsfw = ctx.channel.nsfw.unwrap_or_default();
        let mut user_data = ctx.get_user(&user_id).await?;
        let character_name = ctx.parse_field_required("character-name")?;
        let short_description = ctx.parse_field_required("character-short-description")?;
        let description = ctx.parse_field_required("character-description")?;
        let nsfw_description = ctx.parse_field("character-nsfw-description")?;

        match user_data.characters.entry(character_name.to_owned()) {
            Entry::Vacant(entry) => entry.insert(CharacterProfile {
                short_description: short_description.to_string(),
                description: description.to_string(),
                nsfw_description: nsfw_description.map(|description| description.to_owned()),
                nsfw: nsfw_description.is_some(),
                ..Default::default()
            }),
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                entry.description = description.to_owned();
                entry.short_description = short_description.to_owned();
                if let Some(nsfw_description) = nsfw_description {
                    entry.nsfw = true;
                    entry.nsfw_description = Some(nsfw_description.to_owned());
                }
                entry
            }
        };

        ctx.database.update_user(user_data.clone()).await?;

        let mut embed = ctx.default_embed().await;
        let mut description = format!("{short_description}\n- **Description:**\n{description}");
        if let Some(nsfw_description) = nsfw_description && nsfw {
            writeln!(description, "\n- **NSFW Description:**\n{nsfw_description}")?
        }

        embed.title(format!("Character Profile - {character_name}"));
        embed.description(description);
        embed.author(|a| {
            a.icon_url(user_data.avatar())
                .name(format!("Profile by {}", user_data.name()))
        });

        ctx.respond(|response| response.add_embed(embed)).await
    }

    async fn interaction_component(self, ctx: ComponentInteraction) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        let user_data = ctx.get_user(&ctx.author_id()).await?;
        let name = match self {
            Character::Profile(data) => data.name,
            Character::Create(data) => data.name,
            _ => return ctx.respond(|r| r.content("Invalid command").ephemeral()).await,
        };
        let character = user_data
            .characters
            .get(&name)
            .context("Could not find that character! Was it deleted?")?;
        embed.title(format!("{name}'s Fetishes"));

        let mut fav = String::new();
        let mut love = String::new();
        let mut like = String::new();
        let mut neutral = String::new();
        let mut dislike = String::new();
        let mut hate = String::new();
        let mut limits = String::new();

        for (id, fetish) in &character.fetishes {
            match fetish.category {
                FetishCategory::Favourite => writeln!(fav, "- {id}: {}", fetish.description)?,
                FetishCategory::Love => writeln!(love, "- {id}: {}", fetish.description)?,
                FetishCategory::Like => writeln!(like, "- {id}: {}", fetish.description)?,
                FetishCategory::Neutral => writeln!(neutral, "- {id}: {}", fetish.description)?,
                FetishCategory::Dislike => writeln!(dislike, "- {id}: {}", fetish.description)?,
                FetishCategory::Hate => writeln!(hate, "- {id}: {}", fetish.description)?,
                FetishCategory::Limit => writeln!(limits, "- {id}: {}", fetish.description)?,
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
        let user_id = ctx.author_id();
        let user_data = ctx.get_user(&user_id).await?;
        let choices = match CharacterNameAutocomplete::from_interaction((*ctx.data.clone()).into())?.name {
            AutocompleteValue::None => user_data
                .characters
                .keys()
                .map(|name| CommandOptionChoice {
                    name: name.clone(),
                    name_localizations: None,
                    value: CommandOptionChoiceValue::String(name.clone()),
                })
                .collect(),
            AutocompleteValue::Focused(input) => user_data
                .characters
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