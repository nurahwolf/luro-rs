
mod get_user_characters;
mod get_user_character;
mod update_character_text;
mod update_character;
mod update_character_prefix;
mod get_user_character_prefix;
mod update_character_fetish;
mod get_character_fetishes;
pub struct DbUserCharacter {
    pub character_name: String,
    pub user_id: i64,
    pub nsfw_description: Option<String>,
    pub nsfw_icons: Option<Vec<String>>,
    pub nsfw_summary: Option<String>,
    pub prefix: Option<String>,
    pub sfw_description: String,
    pub sfw_icons: Option<Vec<String>>,
    pub sfw_summary: String,
}

pub struct DbCharacterFetish {
    pub character_name: String,
    pub user_id: i64,
    pub fetish_id: i64,
    pub category: DbUserFetishCategory,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default, Clone, ::sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DbUserFetishCategory {
    Fav,
    Love,
    Like,
    #[default]
    Neutral,
    Dislike,
    Hate,
    Limit
}