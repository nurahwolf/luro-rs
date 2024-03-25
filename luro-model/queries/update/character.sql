INSERT INTO user_characters (
        character_name,
        nsfw_description,
        nsfw_summary,
        prefix,
        sfw_description,
        sfw_summary,
        user_id
) VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT (character_name, user_id)
    DO UPDATE SET
        nsfw_description = $2,
        nsfw_summary = $3,
        prefix = $4,
        sfw_description = $5,
        sfw_summary = $6
