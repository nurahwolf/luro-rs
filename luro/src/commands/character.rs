use anyhow::{Context, Error};
use luro_model::character_profile::{CharacterProfile, FetishCategory};
use std::{collections::btree_map::Entry, fmt::Write};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::{modal::ModalInteractionData, message_component::MessageComponentInteractionData};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use self::{create::Create, profile::Profile};

mod create;
mod profile;

#[derive(CommandModel, CreateCommand)]
#[command(name = "character", desc = "Show off your character!")]
pub enum Character {
    #[command(name = "profile")]
    Profile(Profile),
    #[command(name = "create")]
    Create(Create)
}

impl LuroCommand for Character {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        match self {
            Self::Profile(command) => command.run_command(ctx).await,
            Self::Create(command) => command.run_command(ctx).await
        }
    }

    async fn handle_model(data: ModalInteractionData, ctx: LuroSlash) -> anyhow::Result<()> {
        let user_id = ctx
            .interaction
            .author_id()
            .context("Expected to find the user running this command")?;
        let nsfw = ctx.interaction.clone().channel.unwrap().nsfw.unwrap_or_default();
        let mut user_data = ctx.framework.database.get_user(&user_id).await?;
        let character_name = ctx.parse_modal_field_required(&data, "character-name")?;
        let short_description = ctx.parse_modal_field_required(&data, "character-short-description")?;
        let description = ctx.parse_modal_field_required(&data, "character-description")?;
        let nsfw_description = ctx.parse_modal_field(&data, "character-nsfw-description")?;

        match user_data.characters.entry(character_name.to_owned()) {
            Entry::Vacant(entry) => entry.insert(CharacterProfile {
                short_description: short_description.to_string(),
                description: short_description.to_string(),
                nsfw_description: nsfw_description.map(|description| description.to_owned()),
                nsfw: nsfw_description.is_some(),
                ..Default::default()
            }),
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                entry.description = description.to_owned();
                entry.short_description = short_description.to_owned();
                if let Some(nsfw_description) = nsfw_description {
                    entry.nsfw_description = Some(nsfw_description.to_owned());
                }
                entry
            }
        };

        ctx.framework.database.save_user(&user_id, &user_data).await?;

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

    async fn handle_component(
        self,
        _data: Box<MessageComponentInteractionData>,
        ctx: LuroSlash
    ) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        let message = ctx
        .interaction
        .message
        .clone()
        .ok_or_else(|| Error::msg("Unable to find the original message"))?;
        let user_data = ctx.framework.database.get_user(&message.author.id).await?;
        let name = match self {
            Character::Profile(data) => data.name,
            Character::Create(data) => data.name,
        };
        let character = user_data.characters.get(&name).context("Could not find that character! Was it deleted?")?;
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
            embed.create_field("Love", &fav, false);
        }

        if !like.is_empty() {
            embed.create_field("Like", &fav, false);
        }

        if !neutral.is_empty() {
            embed.create_field("Neutral", &fav, false);
        }

        if !dislike.is_empty() {
            embed.create_field("Dislike", &fav, false);
        }

        if !hate.is_empty() {
            embed.create_field("Hate", &fav, false);
        }

        if !limits.is_empty() {
            embed.create_field("Limits", &fav, false);
        }

        ctx.respond(|r|r.add_embed(embed).ephemeral()).await
    }
}
