INSERT INTO channels (channel_id, guild_id)
VALUES ($1, $2) ON CONFLICT (channel_id) DO
UPDATE
SET channel_id = $1,
    guild_id = $2