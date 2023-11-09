INSERT INTO interactions (
        app_permissions,
        application_id,
        channel_id,
        data,
        guild_id,
        guild_locale,
        interaction_id,
        kind,
        locale,
        message_id,
        token,
        user_id
    )
VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9,
        $10,
        $11,
        $12
    ) ON CONFLICT (interaction_id) DO
UPDATE
SET app_permissions = $1,
    application_id = $2,
    channel_id = $3,
    data = $4,
    guild_id = $5,
    guild_locale = $6,
    interaction_id = $7,
    kind = $8,
    locale = $9,
    message_id = $10,
    user_id = $12