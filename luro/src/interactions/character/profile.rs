use anyhow::Context;
use luro_model::database_driver::LuroDatabaseDriver;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::component::ButtonStyle,
    id::{marker::UserMarker, Id},
};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use super::character_profile;

#[derive(CommandModel, CreateCommand)]
#[command(name = "profile", desc = "Fetch a user's character profile")]
pub struct Profile {
    #[command(desc = "The character to get", autocomplete = true)]
    pub name: String,
    /// Set to true if you wish to see puppeting details for the character
    prefix: Option<bool>,
    /// Include NSFW details? Defaults to the channel type.
    pub nsfw: Option<bool>,
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

        let embed = character_profile(&ctx, character, &user_data, nsfw, self.prefix.unwrap_or_default()).await?;

        ctx.respond(|response| {
            response.add_embed(embed);
            response.components(|components| {
                components.action_row(|row| {
                    if nsfw && !character.fetishes.is_empty() {
                        row.button(|button| {
                            button
                                .custom_id("character-fetish")
                                .label("Fetishes")
                                .style(ButtonStyle::Danger)
                        });
                    }
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
                    })
                })
            });
            response
        })
        .await
    }
}
