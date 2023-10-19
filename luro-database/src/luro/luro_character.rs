use std::sync::Arc;

use crate::{
    sqlx::user_character::{DbUserCharacter, DbCharacterFetish, DbUserFetishCategory},
    LuroDatabase,
};

mod get_fetishes;
mod new;
mod update_fetish;

#[derive(Debug, Clone)]
pub struct LuroCharacter {
    pub db: Arc<LuroDatabase>,
    pub name: String,
    pub nsfw_description: Option<String>,
    pub nsfw_icons: Option<Vec<String>>,
    pub nsfw_summary: Option<String>,
    pub prefix: Option<String>,
    pub sfw_description: String,
    pub sfw_icons: Option<Vec<String>>,
    pub sfw_summary: String,
    pub user_id: i64,
}

pub struct LuroCharacterFetish {
    pub character_name: String,
    pub user_id: i64,
    pub fetish_id: i64,
    pub category: LuroCharacterFetishCategory,
    pub name: String,
    pub description: String,
}

impl std::fmt::Display for LuroCharacterFetishCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuroCharacterFetishCategory::Favourite => write!(f, "Favourite"),
            LuroCharacterFetishCategory::Love => write!(f, "Love"),
            LuroCharacterFetishCategory::Like => write!(f, "Like"),
            LuroCharacterFetishCategory::Neutral => write!(f, "Neutral"),
            LuroCharacterFetishCategory::Dislike => write!(f, "Dislike"),
            LuroCharacterFetishCategory::Hate => write!(f, "Hate"),
            LuroCharacterFetishCategory::Limit => write!(f, "Limit"),
        }
    }
}

#[derive(Default)]
pub enum LuroCharacterFetishCategory {
    Favourite,
    Love,
    Like,
    #[default]
    Neutral,
    Dislike,
    Hate,
    Limit,
}

impl From<DbCharacterFetish> for LuroCharacterFetish {
    fn from(db_fetish: DbCharacterFetish) -> Self {
        Self {
            character_name: db_fetish.character_name,
            user_id: db_fetish.user_id,
            fetish_id: db_fetish.fetish_id,
            category: match db_fetish.category {
                DbUserFetishCategory::Fav => LuroCharacterFetishCategory::Favourite,
                DbUserFetishCategory::Love => LuroCharacterFetishCategory::Love,
                DbUserFetishCategory::Like => LuroCharacterFetishCategory::Like,
                DbUserFetishCategory::Neutral => LuroCharacterFetishCategory::Neutral,
                DbUserFetishCategory::Dislike => LuroCharacterFetishCategory::Dislike,
                DbUserFetishCategory::Hate => LuroCharacterFetishCategory::Hate,
                DbUserFetishCategory::Limit => LuroCharacterFetishCategory::Limit,
            },
            name: db_fetish.name,
            description: db_fetish.description
        }
    }
}

impl From<LuroCharacterFetish> for DbCharacterFetish {
    fn from(db_fetish: LuroCharacterFetish) -> Self {
        Self {
            character_name: db_fetish.character_name,
            user_id: db_fetish.user_id,
            fetish_id: db_fetish.fetish_id,
            category: match db_fetish.category {
                LuroCharacterFetishCategory::Favourite => DbUserFetishCategory::Fav,
                LuroCharacterFetishCategory::Love => DbUserFetishCategory::Love,
                LuroCharacterFetishCategory::Like => DbUserFetishCategory::Like,
                LuroCharacterFetishCategory::Neutral => DbUserFetishCategory::Neutral,
                LuroCharacterFetishCategory::Dislike => DbUserFetishCategory::Dislike,
                LuroCharacterFetishCategory::Hate => DbUserFetishCategory::Hate,
                LuroCharacterFetishCategory::Limit => DbUserFetishCategory::Limit,
            },
            name: db_fetish.name,
            description: db_fetish.description
        }
    }
}

impl From<LuroCharacter> for DbUserCharacter {
    fn from(db_character: LuroCharacter) -> Self {
        Self {
            character_name: db_character.name,
            user_id: db_character.user_id,
            nsfw_description: db_character.nsfw_description,
            nsfw_icons: db_character.nsfw_icons,
            nsfw_summary: db_character.nsfw_summary,
            prefix: db_character.prefix,
            sfw_description: db_character.sfw_description,
            sfw_icons: db_character.sfw_icons,
            sfw_summary: db_character.sfw_summary,
        }
    }
}
