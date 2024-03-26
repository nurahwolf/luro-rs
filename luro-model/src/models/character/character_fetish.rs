use twilight_model::id::{marker::UserMarker, Id};

use super::CharacterFetishCategory;

#[derive(Debug)]
pub struct CharacterFetish {
    pub character_name: String,
    pub user_id: Id<UserMarker>,
    pub fetish_id: i64,
    pub category: CharacterFetishCategory,
    pub name: String,
    pub description: String,
}
