use anyhow::Context;
use luro_model::database_driver::LuroDatabaseDriver;
use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::component::ButtonStyle,
    id::{marker::UserMarker, Id},
};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "profile", desc = "Fetch a user's character profile")]
pub struct Profile {
    #[command(desc = "The character to get", autocomplete = true)]
    pub name: String,
    /// Set to true if you wish to see puppeting details for the character
    prefix: Option<bool>,
    /// Include NSFW details? Defaults to the channel type.
    nsfw: Option<bool>,
    /// Fetch the character from someone else, if you want to see someone elses character!.
    user: Option<Id<UserMarker>>,
}

impl LuroCommand for Profile {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let user_id = match self.user {
            Some(user) => user,
            None => ctx
                .interaction
                .author_id()
                .context("Expected to find the user running this command")?,
        };
        let user_data = ctx.framework.database.get_user(&user_id).await?;
        let interaction_channel_nsfw = &ctx.interaction.clone().channel.unwrap().nsfw;
        let nsfw = match self.nsfw {
            Some(nsfw) => match interaction_channel_nsfw {
                Some(channel_nsfw) => match !channel_nsfw && nsfw {
                    true => {
                        return ctx
                            .respond(|r| r.content("You can't get a NSFW profile in a SFW channel, dork!"))
                            .await
                    }
                    false => nsfw,
                },
                None => nsfw,
            },
            None => interaction_channel_nsfw.unwrap_or_default(),
        };

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

        let mut embed = ctx.default_embed().await;
        let mut description = format!("{}\n", character.short_description);
        if !character.description.is_empty() {
            writeln!(description, "- **Description:**\n{}", character.description)?
        }

        if let Some(nsfw_description) = &character.nsfw_description && nsfw && !nsfw_description.is_empty() {
            writeln!(description, "\n- **NSFW Description:**\n{nsfw_description}")?
        }
        embed.title(format!("Character Profile - {}", self.name));
        embed.description(description);
        embed.author(|a| {
            a.icon_url(user_data.avatar())
                .name(format!("Profile by {}", user_data.name()))
        });

        if self.prefix.unwrap_or_default() {
            let mut prefix_string = String::new();
            for (prefix, character_name) in user_data.character_prefix {
                if self.name == character_name {
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

        {
            let mut rng = thread_rng();
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

        ctx.respond(|response| {
            response.add_embed(embed);
            if nsfw && !character.fetishes.is_empty() {
                response.components(|components| {
                    components.action_row(|row| {
                        row.button(|button| {
                            button
                                .custom_id("character-fetish")
                                .label("Fetishes")
                                .style(ButtonStyle::Danger)
                        })
                    })
                });
            }
            response
        })
        .await
    }
}
