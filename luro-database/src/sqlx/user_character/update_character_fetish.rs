use tracing::info;

use crate::sqlx::user_character::DbUserFetishCategory;
use crate::{sqlx::user_character::DbCharacterFetish, LuroDatabase};

impl LuroDatabase {
    pub async fn update_character_fetish(&self, character: DbCharacterFetish) -> Result<DbCharacterFetish, sqlx::Error> {
        info!("Trying to update character fetish");
        sqlx::query_as!(
            DbCharacterFetish,
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
                category as \"category: DbUserFetishCategory\",
                character_name,
                insert_1.fetish_id,
                user_id, name,
                description FROM insert_2
            JOIN insert_1 ON insert_1.fetish_id = insert_2.fetish_id
            ",
            character.category as _,
            character.character_name,
            character.description,
            character.fetish_id,
            character.name,
            character.user_id,
        )
        .fetch_one(&self.pool)
        .await
    }
}
