SELECT
    guild_roles.colour,
    guild_roles.deleted,
    guild_roles.guild_id,
    guild_roles.hoist,
    guild_roles.icon,
    guild_roles.managed,
    guild_roles.mentionable,
    guild_roles.permissions,
    guild_roles.position,
    guild_roles.role_flags,
    guild_roles.role_id,
    guild_roles.role_name,
    guild_roles.tags as "tags: Json<RoleTags>",
    guild_roles.unicode_emoji
FROM guild_roles
    LEFT JOIN guild_member_roles ON guild_roles.guild_id = guild_member_roles.guild_id AND guild_roles.role_id = guild_member_roles.role_id
WHERE
    guild_member_roles.guild_id = $1 and guild_member_roles.user_id = $2 