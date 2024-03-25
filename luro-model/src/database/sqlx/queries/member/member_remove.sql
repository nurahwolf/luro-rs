INSERT INTO guild_members (
    guild_id,
    left_at,
    user_id
) VALUES ($1, current_timestamp, $2)
ON CONFLICT (guild_id, user_id)
    DO UPDATE SET
        left_at = current_timestamp
