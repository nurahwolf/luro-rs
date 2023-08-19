use std::fmt::Write;
use anyhow::Context;
use luro_model::character_profile::{FetishCategory, Fetish, FetishList};
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{luro_command::LuroCommand, interaction::LuroSlash};

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Add a fetish to a character profile")]
pub struct Add {
    /// The character to modify
    pub name: String,
    /// The fetish type to add
    pub fetish: FetishCategory,
    /// Description of that fetish
    pub description: String,
}

impl LuroCommand for Add {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        let user_id = ctx.interaction.author_id().context("Expected to find the user running this command")?;
        let mut user_data = ctx.framework.database.get_user(&user_id).await?;
        embed.title(format!("Character Profile - {}", self.name));
        embed.author(|a| {
            a.icon_url(user_data.avatar())
                .name(format!("Profile by {}", user_data.name()))
        });


        if user_data.characters.is_empty() {
            return ctx
                .respond(|r| {
                    r.content(format!("Hey <@{user_id}>, you must add a character first!!"))
                        .ephemeral()
                })
                .await;
        }

        let character = match user_data.characters.get_mut(&self.name) {
            Some(character) => {
                character.fetishes.insert(character.fetishes.len() + 1, Fetish {
                    category: self.fetish,
                    description: self.description,
                    list: FetishList::Custom,
                });
                character.clone()
            },
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that you have no characters with the name `{}`! You have the following characters:\n{}", self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };
        
        ctx.framework.database.save_user(&user_id, &user_data).await?;

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