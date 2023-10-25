INSERT INTO guild_roles (
    colour,
    deleted,
    guild_id,
    hoist,
    icon,
    managed,
    mentionable,
    permissions,
    position,
    role_flags,
    role_id,
    role_name,
    tags as \"tags: Json<RoleTags>\",
    unicode_emoji
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
ON CONFLICT (guild_id, role_id)
    DO UPDATE SET
        colour = $1,
        deleted = $2,
        hoist = $3,
        icon = $4,
        managed = $5,
        mentionable = $6,
        permissions = $7,
        position = $8,
        role_flags = $9,
        role_name = $11,
        tags = $12,
        unicode_emoji = $13,