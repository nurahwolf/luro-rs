INSERT INTO guild_roles (guild_id, role_id) VALUES ($1, $2)
ON CONFLICT (guild_id, role_id)
    DO NOTHING