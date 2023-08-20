use anyhow::Context;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::component::ButtonStyle,
    id::{marker::UserMarker, Id}
};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "profile", desc = "Fetch a user's character profile")]
pub struct Profile {
    /// The fursona to get
    pub name: String,
    /// The type of profile to fetch. Defaults to the channel type.
    nsfw: Option<bool>,
    /// Fetch the character name from someone else.
    user: Option<Id<UserMarker>>
}

impl LuroCommand for Profile {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let user_id = match self.user {
            Some(user) => user,
            None => ctx
                .interaction
                .author_id()
                .context("Expected to find the user running this command")?
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
                    false => nsfw
                },
                None => nsfw
            },
            None => interaction_channel_nsfw.unwrap_or_default()
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

        let mut prefix_string = String::new();
        for (prefix, character_name) in user_data.character_prefix {
            if self.name == character_name {
                writeln!(prefix_string, "- `{prefix}`")?
            }
        }
        if !prefix_string.is_empty() {
            embed.create_field("Character Prefixes", &prefix_string, false);
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
