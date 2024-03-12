INSERT INTO user_characters (
        character_name,
        nsfw_description,
        nsfw_icon,
        nsfw_summary,
        prefix,
        sfw_description,
        sfw_icon,
        sfw_summary,
        user_id
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (character_name, user_id)
    DO UPDATE SET
        nsfw_description = $2,
        nsfw_icon = $3,
        nsfw_summary = $4,
        prefix = $5,
        sfw_description = $6,
        sfw_icon = $7,
        sfw_summary = $8