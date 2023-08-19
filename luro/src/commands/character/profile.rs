use anyhow::Context;
use luro_builder::components::{button::ButtonBuilder, ComponentBuilder, action_row::ActionRowBuilder};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::{component::{Button, ButtonStyle}, Component},
    id::{marker::UserMarker, Id}
};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "profile", desc = "Fetch a user's character profile")]
pub struct Profile {
    /// The fursona to get
    pub name: String,
    /// The type of profile to fetch. Defaults to the channel type.
    pub nsfw: Option<bool>,
    /// Fetch the character name from someone else.
    pub user: Option<Id<UserMarker>>
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
        let mut description = format!(
            "{}\n- **Description:**\n{}",
            character.short_description, character.description
        );
        if let Some(nsfw_description) = &character.nsfw_description && nsfw {
            writeln!(description, "\n- **NSFW Description:**\n{nsfw_description}")?
        }
        embed.title(format!("Character Profile - {}", self.name));
        embed.description(description);
        embed.author(|a| {
            a.icon_url(user_data.avatar())
                .name(format!("Profile by {}", user_data.name()))
        });

        let mut buttons = vec![];
        if nsfw && !character.fetishes.is_empty() {
            buttons.push({
                let mut button = ButtonBuilder::default();
                button
                    .custom_id("character-fetish")
                    .label("Fetishes")
                    .style(ButtonStyle::Danger);
                button
            })
        }

        let mut components = vec![];
        let mut action_row = ActionRowBuilder::default();
        let mut buttons_added = 0;
        for button in buttons {
            // If there are too many buttons, break
            if buttons_added > 25 {
                break;
            }

            if buttons_added % 5 == 0 {
                components.push(action_row);
                action_row = ActionRowBuilder::default();
            }

            action_row.button(|b| {
                *b = button;
                b
            });
            buttons_added += 1;
        }
        let components: Vec<Component> = components.iter().map(|component|component.into()).collect();
        ctx.respond(|r| r.add_embed(embed).add_components(components)).await
    }
}
