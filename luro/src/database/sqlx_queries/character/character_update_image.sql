INSERT INTO images (
        character_name,
        favourite,
        img_id,
        name,
        nsfw,
        owner_id,
        source,
        url
    )
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7
    ) ON CONFLICT (img_id) DO
UPDATE
SET
character_name = $1,
favourite = $2,
img_id = $3,
name = $4,
nsfw = $5,
owner_id = $6,
source = $7,
url = $8