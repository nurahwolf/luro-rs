use anyhow::{anyhow, Context, Error};
use luro_builder::embed::{embed_image::EmbedImageBuilder, EmbedBuilder};
use luro_model::{
    database_driver::LuroDatabaseDriver,
    user::{character::{CharacterProfile, FetishCategory}, LuroUser},
};
use rand::seq::SliceRandom;
use std::{collections::btree_map::Entry, fmt::Write};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{application::interaction::{
    message_component::MessageComponentInteractionData, modal::ModalInteractionData,
}, http::interaction::InteractionResponseType};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use self::{create::Create, fetish::Fetish, icon::Icon, profile::Profile, proxy::Proxy, send::CharacterSend};

mod create;
mod fetish;
mod icon;
mod image;
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
    #[command(name = "image")]
    Image(image::Image),
    #[command(name = "send")]
    Send(CharacterSend),
}

impl LuroCommand for Character {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        match self {
            Self::Profile(command) => command.run_command(ctx).await,
            Self::Create(command) => command.run_command(ctx).await,
            Self::Fetish(command) => command.run_command(ctx).await,
            Self::Proxy(command) => command.run_command(ctx).await,
            Self::Icon(command) => command.run_command(ctx).await,
            Self::Image(command) => command.run_command(ctx).await,
            Self::Send(command) => command.run_command(ctx).await,
        }
    }

    async fn handle_model<D: LuroDatabaseDriver>(data: ModalInteractionData, ctx: LuroSlash<D>) -> anyhow::Result<()> {
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

        ctx.framework.database.modify_user(&user_id, &user_data).await?;

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

    async fn handle_component<D: LuroDatabaseDriver>(
        self,
        data: Box<MessageComponentInteractionData>,
        ctx: LuroSlash<D>,
    ) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        let message = ctx
            .interaction
            .message
            .clone()
            .ok_or_else(|| Error::msg("Unable to find the original message"))?;
        let interaction = message
            .interaction
            .context("Unable to get the interaction the original message was attached to")?;
        let user_data = ctx.framework.database.get_user(&interaction.user.id).await?;
        let (nsfw, name) = match self {
            Character::Profile(data) => (data.nsfw.unwrap_or_default(), data.name),
            Character::Create(data) => (false, data.name),
            _ => return ctx.respond(|r| r.content("Invalid command").ephemeral()).await,
        };
        let character = user_data
            .characters
            .get(&name)
            .context("Could not find that character! Was it deleted?")?;

        match data.custom_id.as_str() {
            "character-fetish" => {
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
            "character-image" => {
                let mut embed = message
                    .embeds
                    .first()
                    .context("Expected for there to be an embed in this message!")?
                    .clone();
                let mut sfw_favs = vec![];
                let mut nsfw_favs = vec![];
                for (_, image) in character.images.iter().filter(|(_, img)| img.fav) {
                    match image.nsfw {
                        true => nsfw_favs.push(image),
                        false => sfw_favs.push(image),
                    }
                }

                {
                    let mut rng = rand::thread_rng();
                    if nsfw {
                        if let Some(fav_img) = nsfw_favs.choose(&mut rng) {
                            let mut image = EmbedImageBuilder::default();
                            image.url(fav_img.url.clone());
                            embed.image = Some(image.into());
                        } else if let Some(fav_img) = sfw_favs.choose(&mut rng) {
                            let mut image = EmbedImageBuilder::default();
                            image.url(fav_img.url.clone());
                            embed.image = Some(image.into());
                        }
                    } else if let Some(fav_img) = sfw_favs.choose(&mut rng) {
                        let mut image = EmbedImageBuilder::default();
                        image.url(fav_img.url.clone());
                        embed.image = Some(image.into());
                    }
                }
                
                ctx.respond(|r|r.add_embed(embed).response_type(InteractionResponseType::UpdateMessage)).await

            },
            "character-update" =>{
                let embed = character_profile(&ctx, character, &user_data, nsfw, false).await?;
                ctx.respond(|r|r.add_embed(embed).response_type(InteractionResponseType::UpdateMessage)).await
            },
            name => ctx.internal_error_response(anyhow!("No component named {name} found!")).await,
        }
    }
}

pub async fn character_profile<D: LuroDatabaseDriver>(ctx: &LuroSlash<D>, character: &CharacterProfile, user_data: &LuroUser, nsfw: bool, prefix: bool) -> anyhow::Result<EmbedBuilder> {
    let mut embed = ctx.default_embed().await;
    let mut description = format!("{}\n", character.short_description);
    if !character.description.is_empty() {
        writeln!(description, "- **Description:**\n{}", character.description)?
    }

    if let Some(nsfw_description) = &character.nsfw_description && nsfw && !nsfw_description.is_empty() {
        writeln!(description, "\n- **NSFW Description:**\n{nsfw_description}")?
    }
    embed.title(format!("Character Profile - {}", character.name));
    embed.description(description);
    embed.author(|a| {
        a.icon_url(user_data.avatar())
            .name(format!("Profile by {}", user_data.name()))
    });

    if prefix {
        let mut prefix_string = String::new();
        for (prefix, character_name) in &user_data.character_prefix {
            if &character.name == character_name {
                writeln!(prefix_string, "- `{prefix}`")?
            }
        }
        if !prefix_string.is_empty() {
            embed.create_field("Character Prefixes", &prefix_string, false);
        }
    }

    let mut sfw_favs = vec![];
    let mut nsfw_favs = vec![];
    for (_, image) in character.images.iter().filter(|(_, img)| img.fav) {
        match image.nsfw {
            true => nsfw_favs.push(image),
            false => sfw_favs.push(image),
        }
    }

    if !sfw_favs.is_empty() {
        embed.create_field("SFW Images", &format!("`{}`", sfw_favs.len()), true);
    }

    if !nsfw_favs.is_empty() && nsfw {
        embed.create_field("NSFW Images", &format!("`{}`", nsfw_favs.len()), true);
    }

    {
        let mut rng = rand::thread_rng();
        if nsfw {
            if let Some(fav_img) = nsfw_favs.choose(&mut rng) {
                embed.image(|img| img.url(fav_img.url.clone()));
            } else if let Some(fav_img) = sfw_favs.choose(&mut rng) {
                embed.image(|img| img.url(fav_img.url.clone()));
            }
        } else if let Some(fav_img) = sfw_favs.choose(&mut rng) {
            embed.image(|img| img.url(fav_img.url.clone()));
        }
    }
    
    Ok(embed)
}