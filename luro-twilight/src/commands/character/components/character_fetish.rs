use anyhow::Context;
use luro_framework::{ComponentInteraction, Luro};
use luro_model::types::{CharacterFetishCategory, CommandResponse};
use std::fmt::Write;
use twilight_model::application::interaction::Interaction;

impl crate::commands::character::Character {
    pub async fn character_fetish_button(
        &self,
        ctx: ComponentInteraction,
        invoking_interaction: Interaction,
    ) -> anyhow::Result<CommandResponse> {
        let original_author_id = invoking_interaction
            .author_id()
            .context("Expected to get user ID from interaction")?;
        let character_name = self.character_name();
        let fetishes = ctx
            .database
            .driver
            .character_fetch_fetishes(character_name, original_author_id)
            .await?;

        if fetishes.is_empty() {
            return ctx
                .respond(|r| {
                    r.content(format!(
                        "Whoa! Looks like {character_name} is a vanilla bitch and has no fetishes listed!\nHow plain and boring..."
                    ))
                    .ephemeral()
                })
                .await;
        }

        let character = match ctx.database.user_fetch_character(original_author_id, character_name).await? {
            Some(character) => character,
            None => return ctx.respond(|r|r.content(format!("Sorry, could not find the character {character_name} in my database. The user might have deleted this profile, sorry!")).ephemeral()).await,
        };

        let mut fav = String::new();
        let mut love = String::new();
        let mut like = String::new();
        let mut neutral = String::new();
        let mut dislike = String::new();
        let mut hate = String::new();
        let mut limits = String::new();

        for fetish in fetishes {
            match fetish.category {
                CharacterFetishCategory::Fav => writeln!(fav, "- **{}:** {}", fetish.name, fetish.description)?,
                CharacterFetishCategory::Love => writeln!(love, "- **{}:** {}", fetish.name, fetish.description)?,
                CharacterFetishCategory::Like => writeln!(like, "- **{}:** {}", fetish.name, fetish.description)?,
                CharacterFetishCategory::Neutral => writeln!(neutral, "- **{}:** {}", fetish.name, fetish.description)?,
                CharacterFetishCategory::Dislike => writeln!(dislike, "- **{}:** {}", fetish.name, fetish.description)?,
                CharacterFetishCategory::Hate => writeln!(hate, "- **{}:** {}", fetish.name, fetish.description)?,
                CharacterFetishCategory::Limit => writeln!(limits, "- **{}:** {}", fetish.name, fetish.description)?,
            }
        }

        ctx.respond(|r| {
            r.embed(|embed| {
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
                embed
                    .author(|a| a.name(format!("{character_name}'s Fetishes")).icon_url(character.sfw_icon))
                    .colour(character.colour.unwrap_or(ctx.accent_colour()))
            })
            .ephemeral()
        })
        .await
    }
}
