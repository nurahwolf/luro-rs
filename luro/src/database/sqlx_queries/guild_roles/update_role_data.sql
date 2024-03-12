INSERT INTO guild_roles (
    deleted,
    guild_id,
    role_id
) VALUES ($1, $2, $3)
ON CONFLICT (guild_id, role_id)
    DO UPDATE SET
        deleted = $1