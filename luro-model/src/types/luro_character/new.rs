use std::sync::Arc;

use crate::{sqlx::user_character::DbUserCharacter, LuroCharacter, LuroDatabase};

impl LuroCharacter {
    pub fn new(db_character: DbUserCharacter, db: Arc<LuroDatabase>) -> Self {
        Self {
            db,
            name: db_character.character_name,
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
