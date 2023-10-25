INSERT INTO guilds (
        guild_id,
        owner_id,
        name
    )
VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO
UPDATE
SET owner_id = $2,
    name = $3