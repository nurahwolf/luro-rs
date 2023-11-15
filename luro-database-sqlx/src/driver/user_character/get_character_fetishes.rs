use luro_model::types::{CharacterFetish, CharacterFetishCategory};
use twilight_model::id::{marker::UserMarker, Id};

use crate::driver::user_character::DbCharacterFetishCategory;

impl crate::SQLxDriver {
    pub async fn get_character_fetishes(&self, user_id: Id<UserMarker>, character_name: &str) -> anyhow::Result<Vec<CharacterFetish>> {
        Ok(
            sqlx::query_file!("queries/character_fetch_fetishes.sql", user_id.get() as i64, character_name,)
                .fetch_all(&self.pool)
                .await
                .map(|x| {
                    x.into_iter()
                        .map(|character| CharacterFetish {
                            character_name: character.character_name,
                            user_id,
                            fetish_id: character.fetish_id,
                            category: match character.category {
                                DbCharacterFetishCategory::Fav => CharacterFetishCategory::Fav,
                                DbCharacterFetishCategory::Love => CharacterFetishCategory::Love,
                                DbCharacterFetishCategory::Like => CharacterFetishCategory::Like,
                                DbCharacterFetishCategory::Neutral => CharacterFetishCategory::Neutral,
                                DbCharacterFetishCategory::Dislike => CharacterFetishCategory::Dislike,
                                DbCharacterFetishCategory::Hate => CharacterFetishCategory::Hate,
                                DbCharacterFetishCategory::Limit => CharacterFetishCategory::Limit,
                            },
                            name: character.name,
                            description: character.description,
                        })
                        .collect()
                })?,
        )
    }

    pub async fn get_character_fetish(
        &self,
        user_id: Id<UserMarker>,
        character_name: &str,
        fetish_id: i64,
    ) -> anyhow::Result<Option<CharacterFetish>> {
        Ok(sqlx::query_file!(
            "queries/character_fetch_fetish.sql",
            user_id.get() as i64,
            character_name,
            fetish_id
        )
        .fetch_optional(&self.pool)
        .await
        .map(|x| {
            x.map(|character| CharacterFetish {
                character_name: character.character_name,
                user_id,
                fetish_id: character.fetish_id,
                category: match character.category {
                    DbCharacterFetishCategory::Fav => CharacterFetishCategory::Fav,
                    DbCharacterFetishCategory::Love => CharacterFetishCategory::Love,
                    DbCharacterFetishCategory::Like => CharacterFetishCategory::Like,
                    DbCharacterFetishCategory::Neutral => CharacterFetishCategory::Neutral,
                    DbCharacterFetishCategory::Dislike => CharacterFetishCategory::Dislike,
                    DbCharacterFetishCategory::Hate => CharacterFetishCategory::Hate,
                    DbCharacterFetishCategory::Limit => CharacterFetishCategory::Limit,
                },
                name: character.name,
                description: character.description,
            })
        })?)
    }
}
