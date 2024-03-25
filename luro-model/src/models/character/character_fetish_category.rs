use serde::{Deserialize, Serialize};
use twilight_interactions::command::{CommandOption, CreateOption};
/// The different categories of fetishes a user can have
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Ord, PartialOrd, Eq, CommandOption, CreateOption)]
pub enum FetishCategory {
    #[option(name = "Favourite - Something this character loves to the end of the world", value = "favourite")]
    Favourite,
    #[option(name = "Love - The character loves this!", value = "love")]
    Love,
    #[option(name = "Like - The character likes this", value = "like")]
    Like,
    #[default]
    #[option(name = "Neutral - The character is neutral on this", value = "neutral")]
    Neutral,
    #[option(name = "Dislike - The character dislikes this", value = "dislike")]
    Dislike,
    #[option(name = "Hate - The character hates this", value = "hate")]
    Hate,
    #[option(name = "Limit - A hard no (limit) that this character refuses to do", value = "limit")]
    Limit,
}

#[derive(Debug, Default, Clone, ::sqlx::Type)]
#[sqlx(type_name = "user_characters_fetishes_category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CharacterFetishCategory {
    Fav,
    Love,
    Like,
    #[default]
    Neutral,
    Dislike,
    Hate,
    Limit,
}
