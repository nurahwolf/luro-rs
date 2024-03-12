use anyhow::Context;
use luro_database::LuroCharacterFetish;
use luro_database::LuroCharacterFetishCategory;
use luro_framework::CommandInteraction;
use luro_framework::InteractionTrait;
use luro_framework::{Luro, LuroCommand};
use luro_model::user::character::FetishCategory;
use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "add", desc = "Add an existing fetish to a character profile")]
pub struct Add {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    character: String,
    /// The fetish category to add
    fetish_id: i64,

}

impl LuroCommand for Add {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut embed = ctx.default_embed().await;
        let user = ctx.fetch_user(&ctx.author.user_id).await?;
        embed.title(format!("Character Profile - {}", self.name));
        embed.author(|a| a.icon_url(user.avatar()).name(format!("Profile by {}", user.name())));

        let character = user.fetch_character(&self.name).await?.context("Could not find that character! Was it deleted?")?;
        character.

        character.update_fetish(LuroCharacterFetish {
            character_name: self.character,
            user_id: user.user_id,
            fetish_id: (fetish_total + 1) as i64,
            category: match self.category {
                FetishCategory::Favourite => LuroCharacterFetishCategory::Favourite,
                FetishCategory::Love => LuroCharacterFetishCategory::Love,
                FetishCategory::Like => LuroCharacterFetishCategory::Like,
                FetishCategory::Neutral => LuroCharacterFetishCategory::Neutral,
                FetishCategory::Dislike => LuroCharacterFetishCategory::Dislike,
                FetishCategory::Hate => LuroCharacterFetishCategory::Hate,
                FetishCategory::Limit => LuroCharacterFetishCategory::Limit,
            },
            name: self.name,
            description: self.description,
        }).await?;

        let mut fav = String::new();
        let mut love = String::new();
        let mut like = String::new();
        let mut neutral = String::new();
        let mut dislike = String::new();
        let mut hate = String::new();
        let mut limits = String::new();

        for fetish in &character.get_fetishes().await? {
            match fetish.category {
                LuroCharacterFetishCategory::Favourite => writeln!(fav, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Love => writeln!(love, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Like => writeln!(like, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Neutral => writeln!(neutral, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Dislike => writeln!(dislike, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Hate => writeln!(hate, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
                LuroCharacterFetishCategory::Limit => writeln!(limits, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?,
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
}
