use crate::driver::user_character::DbCharacterFetishCategory;
use luro_model::types::{CharacterFetish, CharacterFetishCategory, CharacterImage, CharacterProfile};
use twilight_model::id::{marker::UserMarker, Id};

impl crate::SQLxDriver {
    pub async fn character_fetch_image(
        &self,
        character_name: &str,
        user_id: Id<UserMarker>,
        image_id: i64,
    ) -> anyhow::Result<Option<CharacterImage>> {
        Ok(sqlx::query_file!(
            "queries/character/character_fetch_image.sql",
            user_id.get() as i64,
            character_name,
            image_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|img| CharacterImage {
            url: img.url,
            nsfw: img.nsfw,
            favourite: img.favourite,
            name: img.name,
            character_name: img.character_name,
            img_id: img.img_id,
            owner_id: img.owner_id,
            source: img.source,
        }))
    }

    pub async fn character_fetch_images(&self, character_name: &str, user_id: Id<UserMarker>) -> anyhow::Result<Vec<CharacterImage>> {
        Ok(
            sqlx::query_file!("queries/character/character_fetch_images.sql", user_id.get() as i64, character_name)
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|img| CharacterImage {
                    url: img.url,
                    nsfw: img.nsfw,
                    favourite: img.favourite,
                    name: img.name,
                    character_name: img.character_name,
                    img_id: img.img_id,
                    owner_id: img.owner_id,
                    source: img.source,
                })
                .collect(),
        )
    }

    pub async fn character_fetch_fetish(
        &self,
        character_name: &str,
        user_id: Id<UserMarker>,
        fetish_id: i64,
    ) -> anyhow::Result<Option<CharacterFetish>> {
        Ok(sqlx::query_file!(
            "queries/character/character_fetch_fetish.sql",
            user_id.get() as i64,
            character_name,
            fetish_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|fetish| CharacterFetish {
            character_name: fetish.character_name,
            user_id,
            fetish_id,
            category: match fetish.category {
                DbCharacterFetishCategory::Fav => CharacterFetishCategory::Fav,
                DbCharacterFetishCategory::Love => CharacterFetishCategory::Love,
                DbCharacterFetishCategory::Like => CharacterFetishCategory::Like,
                DbCharacterFetishCategory::Neutral => CharacterFetishCategory::Neutral,
                DbCharacterFetishCategory::Dislike => CharacterFetishCategory::Dislike,
                DbCharacterFetishCategory::Hate => CharacterFetishCategory::Hate,
                DbCharacterFetishCategory::Limit => CharacterFetishCategory::Limit,
            },
            name: fetish.name,
            description: fetish.description,
        }))
    }

    pub async fn character_update(&self, character: &CharacterProfile, user_id: Id<UserMarker>) -> anyhow::Result<u64> {
        Ok(sqlx::query_file!(
            "queries/character/character_update.sql",
            character.name,
            character.nsfw_description,
            character.nsfw_icon,
            character.nsfw_summary,
            character.prefix,
            character.sfw_description,
            &character.sfw_icon,
            character.sfw_summary,
            user_id.get() as i64,
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }

    pub async fn character_fetch_fetishes(&self, character_name: &str, user_id: Id<UserMarker>) -> anyhow::Result<Vec<CharacterFetish>> {
        Ok(sqlx::query_file!(
            "queries/character/character_fetch_fetishes.sql",
            user_id.get() as i64,
            character_name
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|fetish| CharacterFetish {
            character_name: fetish.character_name,
            user_id,
            fetish_id: fetish.fetish_id,
            category: match fetish.category {
                DbCharacterFetishCategory::Fav => CharacterFetishCategory::Fav,
                DbCharacterFetishCategory::Love => CharacterFetishCategory::Love,
                DbCharacterFetishCategory::Like => CharacterFetishCategory::Like,
                DbCharacterFetishCategory::Neutral => CharacterFetishCategory::Neutral,
                DbCharacterFetishCategory::Dislike => CharacterFetishCategory::Dislike,
                DbCharacterFetishCategory::Hate => CharacterFetishCategory::Hate,
                DbCharacterFetishCategory::Limit => CharacterFetishCategory::Limit,
            },
            name: fetish.name,
            description: fetish.description,
        })
        .collect())
    }

    // pub async fn character_update_image(
    //     &self,
    //     character_name: &str,
    //     user_id: Id<UserMarker>,
    //     img: CharacterImage,
    // ) -> anyhow::Result<u64> {
    //     Ok(sqlx::query_file!(
    //         "queries/character/character_fetch_image.sql",
    //         user_id.get() as i64,
    //         character_name,
    //         image_id
    //     )
    //     .execute(&self.pool)
    //     .await?.rows_updated())
    // }
}
