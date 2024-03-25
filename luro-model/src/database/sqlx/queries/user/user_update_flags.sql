INSERT INTO users (user_flags, user_id)
VALUES ($1, $2) ON CONFLICT (user_id) DO
UPDATE
SET user_flags = $1