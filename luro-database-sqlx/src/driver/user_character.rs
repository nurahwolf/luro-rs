// mod get_character_fetishes;
// mod get_character_images;
// mod get_fetishes;
mod get_user_character;
// mod get_user_character_prefix;
mod get_user_characters;
// mod new_character_image;
// mod update_character;
// mod update_character_fetish;
// mod update_character_image;
// mod update_character_prefix;
// mod update_character_text;

#[derive(Debug, Default, Clone, ::sqlx::Type)]
#[sqlx(type_name = "user_characters_fetishes_category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DbCharacterFetishCategory {
    Fav,
    Love,
    Like,
    #[default]
    Neutral,
    Dislike,
    Hate,
    Limit,
}
