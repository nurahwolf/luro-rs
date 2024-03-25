use futures_util::StreamExt;
use twilight_model::id::Id;

use crate::character::{CharacterFetish, CharacterFetishCategory};

impl crate::database::sqlx::Database {
    pub async fn get_fetishes(&self) -> Result<Vec<CharacterFetish>, sqlx::Error> {
        let mut query = sqlx::query!(
            "
                SELECT
                    category as \"category: CharacterFetishCategory\",
                    character_name,
                    character_fetish.fetish_id,
                    user_id,
                    name,
                    description
                FROM user_characters_fetishes character_fetish
                JOIN fetishes fetish_details ON character_fetish.fetish_id = fetish_details.fetish_id
            "
        )
        .fetch(&self.pool);

        let mut fetishes = vec![];
        while let Some(query) = query.next().await {
            let fetish = match query {
                Ok(data) => data,
                Err(why) => {
                    tracing::warn!(?why, "Failed to query the database");
                    continue;
                }
            };

            fetishes.push(CharacterFetish {
                character_name: fetish.character_name,
                user_id: Id::new(fetish.user_id as u64),
                fetish_id: fetish.fetish_id,
                category: fetish.category,
                name: fetish.name,
                description: fetish.description,
            })
        }

        Ok(fetishes)
    }
}
