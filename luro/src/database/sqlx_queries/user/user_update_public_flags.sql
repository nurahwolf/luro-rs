INSERT INTO users (public_flags, user_id)
VALUES ($1, $2) ON CONFLICT (user_id) DO
UPDATE
SET public_flags = $1