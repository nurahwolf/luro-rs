INSERT INTO users (
    accent_colour,
    bot,
    discriminator,
    email,
    locale,
    mfa_enabled,
    premium_type,
    user_avatar,
    user_banner,
    user_id,
    user_name,
    verified
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
ON CONFLICT (user_id)
    DO UPDATE SET
        accent_colour = $1,
        bot = $2,
        discriminator = $3,
        email = $4,
        locale = $5,
        mfa_enabled = $6,
        premium_type = $7,
        user_avatar = $8,
        user_banner = $9,
        user_name = $11,
        verified = $12