INSERT INTO users (
        user_id,
        user_permissions
    )
VALUES ($1, $2) ON CONFLICT (user_id) DO
UPDATE
SET user_permissions = $2