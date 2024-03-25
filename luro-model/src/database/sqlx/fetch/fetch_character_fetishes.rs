use twilight_model::id::{marker::UserMarker, Id};

use crate::character::{CharacterFetish, CharacterFetishCategory};

impl crate::database::sqlx::Database {
    pub async fn fetch_character_fetishes(
        &self,
        user_id: Id<UserMarker>,
        character_name: &str,
    ) -> Result<Vec<CharacterFetish>, sqlx::Error> {
        Ok(
            sqlx::query_file!("queries/fetch/character_fetishes.sql", user_id.get() as i64, character_name,)
                .fetch_all(&self.pool)
                .await
                .map(|x| {
                    x.into_iter()
                        .map(|character| CharacterFetish {
                            character_name: character.character_name,
                            user_id,
                            fetish_id: character.fetish_id,
                            category: character.category,
                            name: character.name,
                            description: character.description,
                        })
                        .collect()
                })?,
        )
    }

    pub async fn fetch_character_fetish(
        &self,
        user_id: Id<UserMarker>,
        character_name: &str,
        fetish_id: i64,
    ) -> Result<Option<CharacterFetish>, sqlx::Error> {
        Ok(sqlx::query_file!(
            "queries/fetch/character_fetish.sql",
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
                category: character.category,
                name: character.name,
                description: character.description,
            })
        })?)
    }
}
