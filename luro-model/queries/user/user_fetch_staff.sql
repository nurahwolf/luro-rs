SELECT 
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
    users.user_permissions as "user_permissions: UserPermissions",
    users.user_system,
    users.verified,
    users.warnings,
    users.words_average,
    users.words_count,
    users.gender "gender: Gender",
    users.sexuality as "sexuality: Sexuality"
FROM users
WHERE
    user_permissions = 'OWNER' 
        or
    user_permissions = 'ADMINISTRATOR'