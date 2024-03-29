WITH insert_1 AS (
    UPDATE images
    SET name = $4,
        nsfw = $5,
        owner_id = $6,
        source = $7,
        url = $8
    WHERE img_id = $7
    RETURNING *
),
insert_2 AS (
    INSERT INTO user_character_images(
            character_name,
            favourite,
            img_id,
            user_id
        )
    VALUES ($1, $2, $3, $6) ON CONFLICT (user_id, character_name, img_id) DO
    UPDATE
    SET favourite = $2
    RETURNING *
)
SELECT character_name,
    favourite,
    insert_2.img_id,
    name,
    nsfw,
    owner_id,
    source,
    url
FROM insert_2
    JOIN insert_1 ON insert_1.img_id = insert_2.img_id