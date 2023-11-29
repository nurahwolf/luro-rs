INSERT INTO guild_data (
        accent_colour,
        accent_colour_custom,
        guild_id,
        moderator_actions_log_channel
    )
VALUES ($1, $2, $3, $4) ON CONFLICT (guild_id) DO
UPDATE
SET accent_colour = $1,
    accent_colour_custom = $2,
    moderator_actions_log_channel = $4