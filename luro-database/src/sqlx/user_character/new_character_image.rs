use crate::LuroCharacterImage;
use crate::LuroDatabase;

impl LuroDatabase {
    pub async fn new_character_image(&self, img: LuroCharacterImage) -> Result<LuroCharacterImage, sqlx::Error> {
        sqlx::query_as!(
            LuroCharacterImage,
            "
            WITH insert_1 AS (
                INSERT INTO images(name,nsfw,owner_id,source,url)
                VALUES ($3, $4, $5, $6, $7)
                RETURNING
                    *
            ),
            insert_2 AS (
                INSERT INTO user_character_images(character_name, favourite, img_id, user_id)
                SELECT $1, $2, img_id, $5
                FROM insert_1
                ON CONFLICT (user_id, character_name, img_id)
                DO UPDATE SET
                    favourite = $2
                RETURNING
                    character_name,
                    favourite,
                    img_id
            )
            SELECT
                character_name,
                favourite,
                insert_1.img_id,
                name,
                nsfw,
                owner_id,
                source,
                url FROM insert_2
            JOIN insert_1 ON insert_1.img_id = insert_2.img_id
            ",
            img.character_name,
            img.favourite,
            img.name,
            img.nsfw,
            img.owner_id,
            img.source,
            img.url
        )
        .fetch_one(&self.pool)
        .await
    }
}
