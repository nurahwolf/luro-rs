INSERT INTO guild_member_roles (guild_id, role_id, user_id) VALUES ($1, $2, $3)
ON CONFLICT (guild_id, role_id, user_id)
    DO NOTHING