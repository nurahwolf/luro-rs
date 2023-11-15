SELECT character_name,
    favourite,
    images.img_id,
    name,
    nsfw,
    owner_id,
    source,
    url
FROM images
    JOIN user_character_images second ON images.img_id = second.img_id
WHERE (
        user_id = $1
        and character_name = $2
        and images.img_id = $3
    )