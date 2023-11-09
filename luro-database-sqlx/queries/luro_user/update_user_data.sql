INSERT INTO users (
    user_id,
    gender,
    sexuality,
    user_permissions
) VALUES ($1, $2, $3, $4)
ON CONFLICT (user_id)
    DO UPDATE SET
        gender = $2,
        sexuality = $3,
        user_permissions = $4