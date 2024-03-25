INSERT INTO guild_members (
    guild_id,
    left_at,
    user_id
) VALUES ($1, $2, $3)
ON CONFLICT (guild_id, user_id)
    DO UPDATE SET
        left_at = $2