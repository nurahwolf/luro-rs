use twilight_model::id::{marker::UserMarker, Id};

pub struct Character {
    pub character_name: String,
    pub user_id: Id<UserMarker>,
    pub nsfw_description: Option<String>,
    pub nsfw_icons: Option<Vec<String>>,
    pub nsfw_summary: Option<String>,
    pub prefix: Option<String>,
    pub sfw_description: String,
    pub sfw_icons: Option<Vec<String>>,
    pub sfw_summary: String,
}

pub struct CharacterFetish {
    pub character_name: String,
    pub user_id: Id<UserMarker>,
    pub fetish_id: i64,
    pub category: CharacterFetishCategory,
    pub name: String,
    pub description: String,
}

pub enum CharacterFetishCategory {
    Fav,
    Love,
    Like,
    Neutral,
    Dislike,
    Hate,
    Limit,
}