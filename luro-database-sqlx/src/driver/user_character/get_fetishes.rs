use tracing::info;

use super::{DbCharacterFetish, DbUserFetishCategory};

impl crate::SQLxDriver {
    pub async fn get_fetishes(&self) -> Result<Vec<DbCharacterFetish>, sqlx::Error> {
        info!("Trying to get all fetishes");
        sqlx::query_as!(
            DbCharacterFetish,
            "
                SELECT
                    category as \"category: DbUserFetishCategory\",
                    character_name,
                    character_fetish.fetish_id,
                    user_id,
                    name,
                    description
                FROM user_characters_fetishes character_fetish
                JOIN fetishes fetish_details ON character_fetish.fetish_id = fetish_details.fetish_id 
                "
        )
        .fetch_all(&self.pool)
        .await
    }
}
