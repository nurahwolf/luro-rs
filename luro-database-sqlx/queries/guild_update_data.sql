INSERT INTO guilds (
        accent_colour,
        custom_accent_colour,
        guild_id
    )
VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO
UPDATE
SET accent_colour = $1,
    custom_accent_colour = $2