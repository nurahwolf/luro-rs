use anyhow::Context;
use luro_database::LuroCharacterFetish;
use luro_database::LuroCharacterFetishCategory;
use luro_framework::CommandInteraction;

use luro_framework::{Luro, LuroCommand};
use luro_model::user::character::FetishCategory;
use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "create", desc = "Create a fetish and add it to a character profile")]
pub struct Create {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    character: String,
    /// The fetish category to add
    category: FetishCategory,
    /// The name of the fetish
    name: String,
    /// Description of that fetish
    description: String,
}

impl LuroCommand for Create {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await;
        embed.title(format!("Character Profile - {}", self.name));
        embed.author(|a| {
            a.icon_url(ctx.author.avatar_url())
                .name(format!("Profile by {}", ctx.author.name()))
        });

        let character = ctx
            .author
            .fetch_character(ctx.database.clone(), &self.character)
            .await?
            .context("Could not find that character! Was it deleted?")?;
        let fetish_total = ctx.database.get_fetishes().await?.len();

        character
            .update_fetish(LuroCharacterFetish {
                character_name: self.character,
                user_id: ctx.author.user_id.get() as i64,
                fetish_id: fetish_total as i64,
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
            })
            .await?;

        let mut fav = String::new();
        let mut love = String::new();
        let mut like = String::new();
        let mut neutral = String::new();
        let mut dislike = String::new();
        let mut hate = String::new();
        let mut limits = String::new();

        for fetish in &character.fetch_fetishes().await? {
            match fetish.category {
                LuroCharacterFetishCategory::Favourite => {
                    writeln!(fav, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?
                }
                LuroCharacterFetishCategory::Love => {
                    writeln!(love, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?
                }
                LuroCharacterFetishCategory::Like => {
                    writeln!(like, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?
                }
                LuroCharacterFetishCategory::Neutral => {
                    writeln!(neutral, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?
                }
                LuroCharacterFetishCategory::Dislike => {
                    writeln!(dislike, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?
                }
                LuroCharacterFetishCategory::Hate => {
                    writeln!(hate, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?
                }
                LuroCharacterFetishCategory::Limit => {
                    writeln!(limits, "- `{}`: {} - {}", fetish.fetish_id, fetish.name, fetish.description)?
                }
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
