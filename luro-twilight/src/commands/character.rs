use crate::commands::anyhow;
use anyhow::Context;
use async_trait::async_trait;
use luro_framework::{
    command::{LuroCommandBuilder, LuroCommandTrait},
    context::parse_modal_field,
    Framework, InteractionCommand, InteractionComponent, InteractionModal, LuroInteraction
};
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    user::character::{CharacterProfile, FetishCategory}
};
use std::{collections::btree_map::Entry, fmt::Write, sync::Arc};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::InteractionData;

use self::{create::Create, fetish::Fetish, icon::Icon, profile::Profile, proxy::Proxy, send::CharacterSend};

mod create;
mod fetish;
mod icon;
mod profile;
mod proxy;
pub mod send;

#[derive(CommandModel, CreateCommand)]
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
    Send(CharacterSend)
}

impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for Character {}

#[async_trait]
impl LuroCommandTrait for Character {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Arc<Framework<D>>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        match data {
            Self::Profile(_command) => profile::Profile::handle_interaction(ctx, interaction).await,
            Self::Create(_command) => create::Create::handle_interaction(ctx, interaction).await,
            Self::Fetish(command) => command.interaction_command(ctx, interaction).await,
            Self::Proxy(_command) => proxy::Proxy::handle_interaction(ctx, interaction).await,
            Self::Icon(_command) => icon::Icon::handle_interaction(ctx, interaction).await,
            Self::Send(_command) => send::CharacterSend::handle_interaction(ctx, interaction).await
        }
    }

    async fn handle_modal<D: LuroDatabaseDriver>(ctx: Arc<Framework<D>>, interaction: InteractionModal) -> anyhow::Result<()> {
        let user_id = interaction.author_id();
        let nsfw = interaction.clone().channel.unwrap().nsfw.unwrap_or_default();
        let mut user_data = ctx.database.get_user(&user_id).await?;
        let character_name = parse_modal_field::parse_modal_field_required(&interaction.data, "character-name")?;
        let short_description =
            parse_modal_field::parse_modal_field_required(&interaction.data, "character-short-description")?;
        let description = parse_modal_field::parse_modal_field_required(&interaction.data, "character-description")?;
        let nsfw_description = parse_modal_field::parse_modal_field(&interaction.data, "character-nsfw-description")?;

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

        ctx.database.save_user(&user_id, &user_data).await?;

        let mut embed = interaction.default_embed(&ctx).await;
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

        interaction.respond(&ctx, |response| response.add_embed(embed)).await
    }

    async fn handle_component<D: LuroDatabaseDriver>(
        ctx: Arc<Framework<D>>,
        interaction: InteractionComponent
    ) -> anyhow::Result<()> {
        let mut message = interaction.message.clone();
        let mut original_interaction = interaction.original.clone();
        let mut new_id = true;

        while new_id {
            original_interaction = ctx.database.get_interaction(&message.id.to_string()).await?;

            new_id = match original_interaction.message {
                Some(ref new_message) => {
                    message = new_message.clone();
                    true
                }
                None => false
            }
        }

        let command = match original_interaction.data {
            Some(InteractionData::ApplicationCommand(ref data)) => data.clone(),
            _ => {
                return Err(anyhow!(
                    "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                    interaction.data
                ))
            }
        };

        let data = Self::new(command)?;
        let mut embed = interaction.default_embed(&ctx).await;
        let user_data = ctx.database.get_user(&interaction.author_id()).await?;
        let name = match data {
            Character::Profile(data) => data.name,
            Character::Create(data) => data.name,
            _ => return interaction.respond(&ctx, |r| r.content("Invalid command").ephemeral()).await
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
                FetishCategory::Limit => writeln!(limits, "- {id}: {}", fetish.description)?
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

        interaction.respond(&ctx, |r| r.add_embed(embed).ephemeral()).await
    }
}
