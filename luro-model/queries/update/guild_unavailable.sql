INSERT INTO guilds (
        guild_id,
        unavailable
    )
VALUES (
        $1,
        $2
    ) ON CONFLICT (guild_id) DO
UPDATE SET
        unavailable = $2
