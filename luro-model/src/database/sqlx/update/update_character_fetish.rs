use twilight_model::id::Id;

use crate::character::{CharacterFetish, CharacterFetishCategory};

impl crate::database::sqlx::Database {
    pub async fn update_character_fetish(&self, character: CharacterFetish) -> Result<CharacterFetish, sqlx::Error> {
        sqlx::query!(
            "
            WITH insert_1 AS (
                INSERT INTO fetishes(creator,description,fetish_id,name)
                VALUES ($6, $3, $4, $5)
                ON CONFLICT (fetish_id)
                DO UPDATE SET
                    description = $3,
                    creator = $6,
                    name = $5
                RETURNING
                    fetish_id,
                    description,
                    name
            ),
            insert_2 AS (
                INSERT INTO
                    user_characters_fetishes(
                        category,
                        character_name,
                        fetish_id,
                        user_id
                    )
                VALUES ($1, $2, $4, $6)
                ON CONFLICT (user_id, character_name, fetish_id)
                DO UPDATE SET
                    category = $1
                RETURNING
                    category,
                    character_name,
                    fetish_id,
                    user_id
            )
            SELECT
                category as \"category: CharacterFetishCategory\",
                character_name,
                insert_1.fetish_id,
                user_id,
                name,
                description FROM insert_2
            JOIN insert_1 ON insert_1.fetish_id = insert_2.fetish_id
            ",
            character.category as _,
            character.character_name,
            character.description,
            character.fetish_id,
            character.name,
            character.user_id.get() as i64,
        )
        .fetch_one(&self.pool)
        .await
        .map(|fetish| CharacterFetish {
            character_name: fetish.character_name,
            user_id: Id::new(fetish.user_id as u64),
            fetish_id: fetish.fetish_id,
            category: fetish.category,
            name: fetish.name,
            description: fetish.description,
        })
    }
}
