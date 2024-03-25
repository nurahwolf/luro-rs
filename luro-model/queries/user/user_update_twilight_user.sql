INSERT INTO users (
    accent_colour,
    avatar_decoration,
    bot,
    discriminator,
    email,
    global_name,
    locale,
    mfa_enabled,
    premium_type,
    user_avatar,
    user_banner,
    user_id,
    user_name,
    user_system,
    verified
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
ON CONFLICT (user_id)
    DO UPDATE SET
        accent_colour = $1,
        avatar_decoration = $2,
        bot = $3,
        discriminator = $4,
        email = $5,
        global_name = $6,
        locale = $7,
        mfa_enabled = $8,
        premium_type = $9,
        user_avatar = $10,
        user_banner = $11,
        user_name = $13,
        user_system = $14,
        verified = $15