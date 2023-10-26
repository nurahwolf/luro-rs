SELECT
    colour,
    deleted,
    role_flags,
    guild_id,
    hoist,
    icon,
    managed,
    mentionable,
    role_name,
    permissions,
    position,
    role_id,
    tags as "tags: Json<RoleTags>",
    unicode_emoji
FROM guild_roles WHERE guild_id = $1