--! user_fetch : (avatar_decoration?, email?, user_flags?, global_name?, verified?, user_system?, public_flags?, premium_type?, mfa_enabled?, locale?, gender?, sexuality?, user_avatar?, user_banner?, accent_colour?)
SELECT *
FROM users
WHERE user_id = :user_id;