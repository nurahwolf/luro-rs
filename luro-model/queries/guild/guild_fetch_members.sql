SELECT 
    guild_members.boosting_since,
    guild_members.communication_disabled_until,
    guild_members.deafened,
    guild_members.guild_id,
    guild_members.joined_at,
    guild_members.left_at,
    guild_members.guild_owner,
    guild_members.member_avatar,
    guild_members.member_flags,
    guild_members.muted,
    guild_members.nickname,
    guild_members.pending,
    users.accent_colour,
    users.avatar_decoration,
    users.averagesize,
    users.bot,
    users.characters,
    users.discriminator,
    users.email,
    users.global_name,
    users.locale,
    users.message_edits,
    users.messages,
    users.mfa_enabled,
    users.moderation_actions_performed,
    users.moderation_actions,
    users.premium_type,
    users.public_flags,
    users.user_avatar,
    users.user_banner,
    users.user_flags,
    users.user_id,
    users.user_name,
    users.user_permissions as "user_permissions: DbUserPermissions",
    users.gender "gender: DbGender",
    users.sexuality as "sexuality: DbSexuality",
    users.user_system,
    users.verified,
    users.warnings,
    users.words_average,
    users.words_count,
    guilds.owner_id as guild_owner_id,
    guild_roles.permissions as guild_everyone_role_permissions
FROM users
    LEFT JOIN guild_members ON guild_members.user_id = users.user_id
    LEFT JOIN guilds ON guilds.guild_id = guild_members.guild_id
    LEFT JOIN guild_roles ON guild_roles.role_id = guild_members.guild_id
WHERE
    guild_members.guild_id = $1